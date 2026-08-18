//! impl #4 KAT 하네스 — 관용적 safe Rust 구현(mlkem_rs).
//!
//! 프로토콜/사용법은 다른 하네스와 동일:
//!   harness_rs <512|768|1024> <tasks.txt>
//!   harness_rs <512|768|1024> --demo

use std::env;
use std::fs;
use std::process::ExitCode;

use mlkem_rs::ml_kem::{mlkem1024, mlkem512, mlkem768};
use mlkem_rs::kem::KeyError;
use mlkem_rs::params::SYMBYTES;

/// 파라미터 세트별 함수 포인터와 크기를 한 묶음으로.
struct Ops {
    ek_len: usize,
    dk_len: usize,
    ct_len: usize,
    keypair_derand: fn(&mut [u8], &mut [u8], &[u8; 64]),
    enc_derand: fn(&mut [u8], &mut [u8], &[u8], &[u8; 32]) -> Result<(), KeyError>,
    dec: fn(&mut [u8], &[u8], &[u8]) -> Result<(), KeyError>,
    /// FIPS 203 7.2 모듈러스 검사 (단독)
    check_ek: fn(&[u8]) -> Result<(), KeyError>,
    /// FIPS 203 7.3 해시 검사 (단독)
    check_dk: fn(&[u8]) -> Result<(), KeyError>,
}

macro_rules! ops_of {
    ($m:ident) => {
        Ops {
            ek_len: $m::PUBLIC_KEY_BYTES,
            dk_len: $m::SECRET_KEY_BYTES,
            ct_len: $m::CIPHERTEXT_BYTES,
            keypair_derand: $m::keypair_derand,
            enc_derand: $m::enc_derand,
            dec: $m::dec,
            check_ek: $m::check_ek,
            check_dk: $m::check_dk,
        }
    };
}

fn ops_for(v: &str) -> Option<Ops> {
    match v {
        "512" => Some(ops_of!(mlkem512)),
        "768" => Some(ops_of!(mlkem768)),
        "1024" => Some(ops_of!(mlkem1024)),
        _ => None,
    }
}

fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("hex 길이가 홀수: {}", s.len()));
    }
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).map_err(|e| format!("bad hex: {e}")))
        .collect()
}

fn hex_encode(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn run_tasks(ops: &Ops, text: &str) -> Result<String, String> {
    let mut out = String::new();

    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.is_empty() || f[0].starts_with('#') {
            continue;
        }
        if f.len() < 4 {
            return Err(format!("필드 부족: {line}"));
        }
        let (op, id, a, b) = (f[0], f[1], hex_decode(f[2])?, hex_decode(f[3])?);

        match op {
            "KEYGEN" => {
                if a.len() != SYMBYTES || b.len() != SYMBYTES {
                    return Err(format!("KEYGEN {id}: d/z 는 32바이트여야 함"));
                }
                let mut coins = [0u8; 64];
                coins[..SYMBYTES].copy_from_slice(&a);
                coins[SYMBYTES..].copy_from_slice(&b);

                let mut ek = vec![0u8; ops.ek_len];
                let mut dk = vec![0u8; ops.dk_len];
                (ops.keypair_derand)(&mut ek, &mut dk, &coins);
                out.push_str(&format!("KEYGEN {id} {} {}\n", hex_encode(&ek), hex_encode(&dk)));
            }
            "ENCAPS" => {
                if a.len() != ops.ek_len || b.len() != SYMBYTES {
                    return Err(format!("ENCAPS {id}: 크기 불일치 (ek={} m={})", a.len(), b.len()));
                }
                let m: &[u8; 32] = b.as_slice().try_into().map_err(|_| "m 크기 오류")?;
                let mut ct = vec![0u8; ops.ct_len];
                let mut ss = [0u8; 32];
                (ops.enc_derand)(&mut ct, &mut ss, &a, m)
                    .map_err(|e| format!("ENCAPS {id}: 키 검사 실패 {e:?}"))?;
                out.push_str(&format!("ENCAPS {id} {} {}\n", hex_encode(&ct), hex_encode(&ss)));
            }
            "DECAPS" => {
                if a.len() != ops.dk_len || b.len() != ops.ct_len {
                    return Err(format!("DECAPS {id}: 크기 불일치 (dk={} ct={})", a.len(), b.len()));
                }
                let mut ss = [0u8; 32];
                let rc = match (ops.dec)(&mut ss, &b, &a) {
                    Ok(()) => 0,
                    // 키 검사에서 걸러진 경우. 변조된 '암호문' 은 여기서 걸리지 않고
                    // 암묵적 거부로 처리되므로 rc 는 0 이다.
                    Err(_) => 1,
                };
                out.push_str(&format!("DECAPS {id} {} {rc}\n", hex_encode(&ss)));
            }
            // 키 유효성 검사: rc 0 = 통과(유효), 1 = 거부
            "CHECKEK" => {
                let rc = if (ops.check_ek)(&a).is_ok() { 0 } else { 1 };
                out.push_str(&format!("CHECKEK {id} {rc} 0\n"));
            }
            "CHECKDK" => {
                let rc = if (ops.check_dk)(&a).is_ok() { 0 } else { 1 };
                out.push_str(&format!("CHECKDK {id} {rc} 0\n"));
            }
            _ => return Err(format!("알 수 없는 op: {op}")),
        }
    }
    Ok(out)
}

fn demo(ops: &Ops, variant: &str) -> Result<(), String> {
    use rand::RngCore;
    let mut coins = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut coins);
    let mut m = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut m);

    let mut ek = vec![0u8; ops.ek_len];
    let mut dk = vec![0u8; ops.dk_len];
    let mut ct = vec![0u8; ops.ct_len];
    let (mut ss_a, mut ss_b) = ([0u8; 32], [0u8; 32]);

    (ops.keypair_derand)(&mut ek, &mut dk, &coins);
    (ops.enc_derand)(&mut ct, &mut ss_a, &ek, &m).map_err(|e| format!("{e:?}"))?;
    (ops.dec)(&mut ss_b, &ct, &dk).map_err(|e| format!("{e:?}"))?;

    println!("ML-KEM-{variant} 랜덤 왕복 데모 (safe Rust 리팩터링)");
    println!("  ek {} B, dk {} B, ct {} B", ops.ek_len, ops.dk_len, ops.ct_len);
    println!("  encaps 공유키: {}", hex_encode(&ss_a));
    println!("  decaps 공유키: {}", hex_encode(&ss_b));
    if ss_a == ss_b {
        println!("  => 일치: 정상 동작");
        Ok(())
    } else {
        Err("공유키 불일치!".into())
    }
}


/// 벤치 상태: 출력 버퍼 묶음.
struct BenchState {
    ek: Vec<u8>,
    dk: Vec<u8>,
    ct: Vec<u8>,
    ct_bad: Vec<u8>,
    ss: [u8; 32],
}

/// 4연산(keygen / encaps / decaps / reject) 벤치.
/// 입력 패턴·워밍업·체크섬은 bench_util 로 네 구현이 동일하게 맞춰져 있다.
fn bench(ops: &Ops, iters: usize) {
    let coins0 = bench_util::base_coins();
    let msg0 = bench_util::base_msg();

    let mut st = BenchState {
        ek: vec![0u8; ops.ek_len],
        dk: vec![0u8; ops.dk_len],
        ct: vec![0u8; ops.ct_len],
        ct_bad: vec![0u8; ops.ct_len],
        ss: [0u8; 32],
    };

    // 고정 키쌍 / 암호문 준비
    (ops.keypair_derand)(&mut st.ek, &mut st.dk, &coins0);
    let (mut ct0, mut ss0) = (vec![0u8; ops.ct_len], [0u8; 32]);
    (ops.enc_derand)(&mut ct0, &mut ss0, &st.ek, &msg0).expect("encaps");
    st.ct.copy_from_slice(&ct0);
    st.ct_bad.copy_from_slice(&ct0);
    st.ct_bad[0] ^= 0x01; // 변조 → 암묵적 거부 경로

    bench_util::timed(
        "keygen", iters, &mut st,
        |s, i| {
            let mut coins = coins0;
            coins[0] = (i & 0xFF) as u8;
            (ops.keypair_derand)(&mut s.ek, &mut s.dk, &coins);
        },
        |s, acc| { acc.fold(&s.ek); acc.fold(&s.dk); },
    );

    // 고정 키쌍으로 되돌린다
    (ops.keypair_derand)(&mut st.ek, &mut st.dk, &coins0);

    bench_util::timed(
        "encaps", iters, &mut st,
        |s, i| {
            let mut m = msg0;
            m[0] = (i & 0xFF) as u8;
            let mut ss = [0u8; 32];
            (ops.enc_derand)(&mut s.ct, &mut ss, &s.ek, &m).expect("encaps");
            s.ss = ss;
        },
        |s, acc| { acc.fold(&s.ct); acc.fold(&s.ss); },
    );

    // 고정 암호문으로 되돌린다
    (ops.enc_derand)(&mut st.ct, &mut st.ss, &st.ek.clone(), &msg0).expect("encaps");
    st.ct_bad.copy_from_slice(&st.ct);
    st.ct_bad[0] ^= 0x01;

    bench_util::timed(
        "decaps", iters, &mut st,
        |s, _| {
            let mut ss = [0u8; 32];
            (ops.dec)(&mut ss, &s.ct, &s.dk).expect("decaps");
            s.ss = ss;
        },
        |s, acc| acc.fold(&s.ss),
    );

    bench_util::timed(
        "reject", iters, &mut st,
        |s, _| {
            let mut ss = [0u8; 32];
            (ops.dec)(&mut ss, &s.ct_bad, &s.dk).expect("reject");
            s.ss = ss;
        },
        |s, acc| acc.fold(&s.ss),
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <512|768|1024> <tasks.txt|--demo|--bench [iters]>", args[0]);
        return ExitCode::from(2);
    }
    let Some(ops) = ops_for(&args[1]) else {
        eprintln!("알 수 없는 파라미터 세트: {}", args[1]);
        return ExitCode::from(2);
    };

    if args[2] == "--bench" {
        let iters: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(100);
        bench(&ops, iters);
        return ExitCode::SUCCESS;
    }

    if args[2] == "--demo" {
        return match demo(&ops, &args[1]) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => { eprintln!("데모 실패: {e}"); ExitCode::from(1) }
        };
    }

    let text = match fs::read_to_string(&args[2]) {
        Ok(t) => t,
        Err(e) => { eprintln!("{} 를 읽을 수 없음: {e}", args[2]); return ExitCode::from(2); }
    };
    match run_tasks(&ops, &text) {
        Ok(out) => { print!("{out}"); ExitCode::SUCCESS }
        Err(e) => { eprintln!("오류: {e}"); ExitCode::from(1) }
    }
}
