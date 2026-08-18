//! impl #1: C2Rust 로 변환된 liboqs ML-KEM ref 구현 KAT 하네스.
//!
//! FIPS 203 내부 결정적 함수를 그대로 호출한다.
//!   KeyGen_internal : *_keypair_derand(pk, sk, coins = d||z)
//!   Encaps_internal : *_enc_derand(ct, ss, pk, coins = m)
//!   Decaps          : *_dec(ss, ct, sk)     (정상/변조 동일 호출)
//!
//! 세 구현이 공유하는 텍스트 프로토콜:
//!   입력: KEYGEN <id> <d_hex> <z_hex>
//!         ENCAPS <id> <ek_hex> <m_hex>
//!         DECAPS <id> <dk_hex> <c_hex>
//!   출력: KEYGEN <id> <ek_hex> <dk_hex>
//!         ENCAPS <id> <c_hex> <k_hex>
//!         DECAPS <id> <k_hex> <rc>
//!
//! 사용법: harness_c2rust <512|768|1024> <tasks.txt>
//!         harness_c2rust <512|768|1024> --demo     (랜덤 왕복 데모)

use std::env;
use std::fs;
use std::process::ExitCode;

/// 변환된 각 크레이트의 FFI 진입점과 크기 상수를 하나로 묶는다.
struct Ops {
    ek_len: usize,
    dk_len: usize,
    ct_len: usize,
    keypair_derand: unsafe extern "C" fn(*mut u8, *mut u8, *const u8) -> i32,
    enc_derand: unsafe extern "C" fn(*mut u8, *mut u8, *const u8, *const u8) -> i32,
    dec: unsafe extern "C" fn(*mut u8, *const u8, *const u8) -> i32,
    /// 랜덤 왕복 데모용(내부에서 OQS_randombytes -> rand crate shim 호출)
    keypair: unsafe extern "C" fn(*mut u8, *mut u8) -> i32,
    enc: unsafe extern "C" fn(*mut u8, *mut u8, *const u8) -> i32,
    /// FIPS 203 7.2 모듈러스 검사 (단독)
    check_pk: unsafe extern "C" fn(*const u8) -> i32,
    /// FIPS 203 7.3 해시 검사 (단독)
    check_sk: unsafe extern "C" fn(*const u8) -> i32,
}

const SS_LEN: usize = 32;
const SYM: usize = 32;

fn ops_for(variant: &str) -> Option<Ops> {
    use mlkem1024_c2rust::src::kem as k1024;
    use mlkem512_c2rust::src::kem as k512;
    use mlkem768_c2rust::src::kem as k768;

    match variant {
        "512" => Some(Ops {
            ek_len: 800,
            dk_len: 1632,
            ct_len: 768,
            keypair_derand: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_keypair_derand,
            enc_derand: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_enc_derand,
            dec: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_dec,
            keypair: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_keypair,
            enc: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_enc,
            check_pk: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_check_pk,
            check_sk: k512::PQCP_MLKEM_NATIVE_MLKEM512_C_check_sk,
        }),
        "768" => Some(Ops {
            ek_len: 1184,
            dk_len: 2400,
            ct_len: 1088,
            keypair_derand: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_keypair_derand,
            enc_derand: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_enc_derand,
            dec: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_dec,
            keypair: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_keypair,
            enc: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_enc,
            check_pk: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_check_pk,
            check_sk: k768::PQCP_MLKEM_NATIVE_MLKEM768_C_check_sk,
        }),
        "1024" => Some(Ops {
            ek_len: 1568,
            dk_len: 3168,
            ct_len: 1568,
            keypair_derand: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_keypair_derand,
            enc_derand: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_enc_derand,
            dec: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_dec,
            keypair: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_keypair,
            enc: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_enc,
            check_pk: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_check_pk,
            check_sk: k1024::PQCP_MLKEM_NATIVE_MLKEM1024_C_check_sk,
        }),
        _ => None,
    }
}

fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("hex 길이가 홀수: {}", s.len()));
    }
    (0..s.len() / 2)
        .map(|i| {
            u8::from_str_radix(&s[2 * i..2 * i + 2], 16).map_err(|e| format!("bad hex: {e}"))
        })
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
                if a.len() != SYM || b.len() != SYM {
                    return Err(format!("KEYGEN {id}: d/z 는 32바이트여야 함"));
                }
                // coins = d || z
                let mut coins = [0u8; 2 * SYM];
                coins[..SYM].copy_from_slice(&a);
                coins[SYM..].copy_from_slice(&b);

                let mut ek = vec![0u8; ops.ek_len];
                let mut dk = vec![0u8; ops.dk_len];
                let rc = unsafe {
                    (ops.keypair_derand)(ek.as_mut_ptr(), dk.as_mut_ptr(), coins.as_ptr())
                };
                if rc != 0 {
                    return Err(format!("KEYGEN {id}: rc={rc}"));
                }
                out.push_str(&format!(
                    "KEYGEN {id} {} {}\n",
                    hex_encode(&ek),
                    hex_encode(&dk)
                ));
            }
            "ENCAPS" => {
                if a.len() != ops.ek_len || b.len() != SYM {
                    return Err(format!(
                        "ENCAPS {id}: 크기 불일치 (ek={} m={})",
                        a.len(),
                        b.len()
                    ));
                }
                let mut ct = vec![0u8; ops.ct_len];
                let mut ss = [0u8; SS_LEN];
                let rc = unsafe {
                    (ops.enc_derand)(
                        ct.as_mut_ptr(),
                        ss.as_mut_ptr(),
                        a.as_ptr(),
                        b.as_ptr(),
                    )
                };
                if rc != 0 {
                    return Err(format!("ENCAPS {id}: rc={rc}"));
                }
                out.push_str(&format!(
                    "ENCAPS {id} {} {}\n",
                    hex_encode(&ct),
                    hex_encode(&ss)
                ));
            }
            "DECAPS" => {
                if a.len() != ops.dk_len || b.len() != ops.ct_len {
                    return Err(format!(
                        "DECAPS {id}: 크기 불일치 (dk={} ct={})",
                        a.len(),
                        b.len()
                    ));
                }
                // ML-KEM 은 변조 암호문에도 에러를 내지 않는다(암묵적 거부).
                // z 기반 의사난수 공유키를 반환하고 rc=0 이다.
                let mut ss = [0u8; SS_LEN];
                let rc = unsafe { (ops.dec)(ss.as_mut_ptr(), b.as_ptr(), a.as_ptr()) };
                out.push_str(&format!("DECAPS {id} {} {rc}\n", hex_encode(&ss)));
            }
            // 키 유효성 검사: rc 0 = 통과(유효), 그 외 = 거부
            // FIPS 203 입력 검증은 두 단계다.
            //   1) 타입 검사: 길이가 규격과 정확히 같은가
            //   2) 내용 검사: 모듈러스(7.2) 또는 H(ek) 해시(7.3)
            // 길이가 다르면 내용 검사를 하지 않고 바로 거부한다.
            "CHECKEK" => {
                let rc = if a.len() != ops.ek_len {
                    1
                } else {
                    i32::from(unsafe { (ops.check_pk)(a.as_ptr()) } != 0)
                };
                out.push_str(&format!("CHECKEK {id} {rc} 0\n"));
            }
            "CHECKDK" => {
                let rc = if a.len() != ops.dk_len {
                    1
                } else {
                    i32::from(unsafe { (ops.check_sk)(a.as_ptr()) } != 0)
                };
                out.push_str(&format!("CHECKDK {id} {rc} 0\n"));
            }
            _ => return Err(format!("알 수 없는 op: {op}")),
        }
    }
    Ok(out)
}

/// 랜덤 왕복 데모: 변환된 코드가 실제로 "돌아간다"는 것을 보여준다.
/// 여기서만 OQS_randombytes(-> rand crate shim)가 사용된다.
fn demo(ops: &Ops, variant: &str) -> Result<(), String> {
    let mut ek = vec![0u8; ops.ek_len];
    let mut dk = vec![0u8; ops.dk_len];
    let mut ct = vec![0u8; ops.ct_len];
    let mut ss_a = [0u8; SS_LEN];
    let mut ss_b = [0u8; SS_LEN];

    let rc = unsafe { (ops.keypair)(ek.as_mut_ptr(), dk.as_mut_ptr()) };
    if rc != 0 {
        return Err(format!("keypair rc={rc}"));
    }
    let rc = unsafe { (ops.enc)(ct.as_mut_ptr(), ss_a.as_mut_ptr(), ek.as_ptr()) };
    if rc != 0 {
        return Err(format!("enc rc={rc}"));
    }
    let rc = unsafe { (ops.dec)(ss_b.as_mut_ptr(), ct.as_ptr(), dk.as_ptr()) };
    if rc != 0 {
        return Err(format!("dec rc={rc}"));
    }

    println!("ML-KEM-{variant} 랜덤 왕복 데모 (C2Rust)");
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
    ss: [u8; SS_LEN],
}

/// 4연산(keygen / encaps / decaps / reject) 벤치.
///
/// 측정 대상은 KAT 과 같은 결정적 내부 함수들이다. 입력 패턴·워밍업·체크섬은
/// bench_util 을 통해 네 구현이 동일하게 맞춰져 있다.
fn bench(ops: &Ops, iters: usize) -> Result<(), String> {
    let coins0 = bench_util::base_coins();
    let msg0 = bench_util::base_msg();

    let mut st = BenchState {
        ek: vec![0u8; ops.ek_len],
        dk: vec![0u8; ops.dk_len],
        ct: vec![0u8; ops.ct_len],
        ct_bad: vec![0u8; ops.ct_len],
        ss: [0u8; SS_LEN],
    };

    // 고정 키쌍 / 암호문 준비 (encaps·decaps 벤치 입력)
    let rc = unsafe {
        (ops.keypair_derand)(st.ek.as_mut_ptr(), st.dk.as_mut_ptr(), coins0.as_ptr())
    };
    if rc != 0 {
        return Err(format!("keygen rc={rc}"));
    }
    let rc = unsafe {
        (ops.enc_derand)(
            st.ct.as_mut_ptr(),
            st.ss.as_mut_ptr(),
            st.ek.as_ptr(),
            msg0.as_ptr(),
        )
    };
    if rc != 0 {
        return Err(format!("encaps rc={rc}"));
    }
    st.ct_bad.copy_from_slice(&st.ct);
    st.ct_bad[0] ^= 0x01; // 변조 → 암묵적 거부 경로

    // ---- keygen ----
    bench_util::timed(
        "keygen",
        iters,
        &mut st,
        |s, i| {
            let mut coins = coins0;
            coins[0] = (i & 0xFF) as u8;
            unsafe { (ops.keypair_derand)(s.ek.as_mut_ptr(), s.dk.as_mut_ptr(), coins.as_ptr()) };
        },
        |s, acc| {
            acc.fold(&s.ek);
            acc.fold(&s.dk);
        },
    );

    // 고정 키쌍으로 되돌린다
    unsafe { (ops.keypair_derand)(st.ek.as_mut_ptr(), st.dk.as_mut_ptr(), coins0.as_ptr()) };

    // ---- encaps ----
    bench_util::timed(
        "encaps",
        iters,
        &mut st,
        |s, i| {
            let mut m = msg0;
            m[0] = (i & 0xFF) as u8;
            unsafe {
                (ops.enc_derand)(s.ct.as_mut_ptr(), s.ss.as_mut_ptr(), s.ek.as_ptr(), m.as_ptr())
            };
        },
        |s, acc| {
            acc.fold(&s.ct);
            acc.fold(&s.ss);
        },
    );

    // 고정 암호문으로 되돌린다
    unsafe {
        (ops.enc_derand)(
            st.ct.as_mut_ptr(),
            st.ss.as_mut_ptr(),
            st.ek.as_ptr(),
            msg0.as_ptr(),
        )
    };
    st.ct_bad.copy_from_slice(&st.ct);
    st.ct_bad[0] ^= 0x01;

    // ---- decaps (정상) ----
    bench_util::timed(
        "decaps",
        iters,
        &mut st,
        |s, _| {
            unsafe { (ops.dec)(s.ss.as_mut_ptr(), s.ct.as_ptr(), s.dk.as_ptr()) };
        },
        |s, acc| acc.fold(&s.ss),
    );

    // ---- decaps (변조 → 암묵적 거부) ----
    bench_util::timed(
        "reject",
        iters,
        &mut st,
        |s, _| {
            unsafe { (ops.dec)(s.ss.as_mut_ptr(), s.ct_bad.as_ptr(), s.dk.as_ptr()) };
        },
        |s, acc| acc.fold(&s.ss),
    );

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <512|768|1024> <tasks.txt|--demo|--bench [iters]>", args[0]);
        return ExitCode::from(2);
    }
    let variant = &args[1];
    let Some(ops) = ops_for(variant) else {
        eprintln!("알 수 없는 파라미터 세트: {variant}");
        return ExitCode::from(2);
    };

    if args[2] == "--bench" {
        let iters: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(100);
        return match bench(&ops, iters) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("벤치 실패: {e}");
                ExitCode::from(1)
            }
        };
    }

    if args[2] == "--demo" {
        return match demo(&ops, variant) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("데모 실패: {e}");
                ExitCode::from(1)
            }
        };
    }

    let text = match fs::read_to_string(&args[2]) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} 를 읽을 수 없음: {e}", args[2]);
            return ExitCode::from(2);
        }
    };
    match run_tasks(&ops, &text) {
        Ok(out) => {
            print!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("오류: {e}");
            ExitCode::from(1)
        }
    }
}
