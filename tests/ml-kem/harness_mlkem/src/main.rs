//! impl #3: 오픈소스 순수 Rust ML-KEM (RustCrypto `ml-kem`) KAT 하네스.
//!
//! FIPS 203 내부 결정적 함수 대응:
//!   KeyGen_internal : DecapsulationKey::from_seed(d || z)
//!   Encaps_internal : EncapsulationKey::encapsulate_deterministic(m)
//!   Decaps          : DecapsulationKey::decapsulate(c)   (정상/변조 동일 호출)
//!
//! dk 는 ACVP 와 비교하기 위해 legacy "expanded" 형식으로 직렬화한다
//! (ml-kem 0.3 에서 deprecated 지만 ACVP 벡터가 이 형식을 쓴다).
//!
//! 프로토콜/사용법은 다른 두 하네스와 동일:
//!   harness_mlkem <512|768|1024> <tasks.txt>
//!   harness_mlkem <512|768|1024> --demo

#![allow(deprecated)] // ExpandedKeyEncoding: ACVP 벡터 형식과 맞추기 위해 필요

use std::env;
use std::fs;
use std::process::ExitCode;

use ml_kem::array::Array;
use ml_kem::{
    Ciphertext, Decapsulate, DecapsulationKey, EncapsulationKey, ExpandedDecapsulationKey,
    ExpandedKeyEncoding, Key, KeyExport, MlKem512, MlKem768, MlKem1024, Seed,
};

const SYM: usize = 32;

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

/// 파라미터 세트별 구현을 동일한 코드로 생성한다.
macro_rules! variant_impl {
    ($modname:ident, $P:ty) => {
        mod $modname {
            use super::*;

            pub fn keygen(d: &[u8], z: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
                let mut buf = [0u8; 64];
                buf[..SYM].copy_from_slice(d);
                buf[SYM..].copy_from_slice(z);
                let seed = Seed::try_from(&buf[..]).map_err(|_| "seed 길이 오류".to_string())?;

                let dk = DecapsulationKey::<$P>::from_seed(seed);
                let ek = dk.encapsulation_key().to_bytes().to_vec();
                let dkb = dk.to_expanded_bytes().to_vec();
                Ok((ek, dkb))
            }

            pub fn encaps(ek_bytes: &[u8], m: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
                let key = Key::<EncapsulationKey<$P>>::try_from(ek_bytes)
                    .map_err(|_| format!("ek 길이 오류: {}", ek_bytes.len()))?;
                let ek = EncapsulationKey::<$P>::new(&key)
                    .map_err(|_| "ek 검증 실패".to_string())?;

                let m32 = Array::try_from(m).map_err(|_| "m 은 32바이트여야 함".to_string())?;
                let (ct, k) = ek.encapsulate_deterministic(&m32);
                Ok((ct.to_vec(), k.to_vec()))
            }

            /// FIPS 203 7.2 모듈러스 검사에 해당.
            /// ml-kem 은 EncapsulationKey 생성 시 이 검사를 수행하고 실패를 반환한다.
            pub fn check_ek(ek_bytes: &[u8]) -> bool {
                match Key::<EncapsulationKey<$P>>::try_from(ek_bytes) {
                    Ok(k) => EncapsulationKey::<$P>::new(&k).is_ok(),
                    Err(_) => false,
                }
            }

            /// FIPS 203 7.3 해시 검사에 해당.
            /// ml-kem 은 확장 형식 dk 를 읽을 때 H(ek) 를 다시 계산해 대조한다.
            pub fn check_dk(dk_bytes: &[u8]) -> bool {
                match ExpandedDecapsulationKey::<$P>::try_from(dk_bytes) {
                    Ok(e) => DecapsulationKey::<$P>::from_expanded_bytes(&e).is_ok(),
                    Err(_) => false,
                }
            }

            pub fn decaps(dk_bytes: &[u8], ct_bytes: &[u8]) -> Result<Vec<u8>, String> {
                let enc = ExpandedDecapsulationKey::<$P>::try_from(dk_bytes)
                    .map_err(|_| format!("dk 길이 오류: {}", dk_bytes.len()))?;
                let dk = DecapsulationKey::<$P>::from_expanded_bytes(&enc)
                    .map_err(|_| "dk 검증 실패".to_string())?;

                let ct = Ciphertext::<$P>::try_from(ct_bytes)
                    .map_err(|_| format!("ct 길이 오류: {}", ct_bytes.len()))?;
                // 변조된 ct 여도 에러가 아니라 암묵적 거부(의사난수 키)로 처리된다.
                Ok(dk.decapsulate(&ct).to_vec())
            }


            /// 4연산 벤치. 다른 구현과 동일한 입력/워밍업/체크섬을 쓴다.
            ///
            /// 주의: ml-kem 은 객체 기반 API 이므로 keygen 이외의 연산에서
            /// 키 역직렬화(+검증)가 측정 구간에 포함된다. 다른 구현도 매 호출마다
            /// 바이트에서 키를 파싱하므로 "바이트 in → 바이트 out" 기준으로는
            /// 동일한 사용자 수준 연산을 비교하는 것이다.
            pub fn bench(variant: &str, iters: usize) -> Result<(), String> {
                let _ = variant;
                let coins0 = bench_util::base_coins();
                let msg0 = bench_util::base_msg();

                struct St {
                    ek: Vec<u8>,
                    dk: Vec<u8>,
                    ct: Vec<u8>,
                    ct_bad: Vec<u8>,
                    ss: Vec<u8>,
                }

                let (ek0, dk0) = keygen(&coins0[..32], &coins0[32..])?;
                let (ct0, ss0) = encaps(&ek0, &msg0)?;
                let mut ct_bad = ct0.clone();
                ct_bad[0] ^= 0x01;

                let mut st = St {
                    ek: ek0.clone(),
                    dk: dk0.clone(),
                    ct: ct0.clone(),
                    ct_bad,
                    ss: ss0,
                };

                bench_util::timed(
                    "keygen", iters, &mut st,
                    |s, i| {
                        let mut coins = coins0;
                        coins[0] = (i & 0xFF) as u8;
                        let (ek, dk) = keygen(&coins[..32], &coins[32..]).expect("keygen");
                        s.ek = ek;
                        s.dk = dk;
                    },
                    |s, acc| { acc.fold(&s.ek); acc.fold(&s.dk); },
                );

                // 고정 키쌍으로 되돌린다
                st.ek = ek0.clone();
                st.dk = dk0.clone();

                bench_util::timed(
                    "encaps", iters, &mut st,
                    |s, i| {
                        let mut m = msg0;
                        m[0] = (i & 0xFF) as u8;
                        let (ct, ss) = encaps(&s.ek, &m).expect("encaps");
                        s.ct = ct;
                        s.ss = ss;
                    },
                    |s, acc| { acc.fold(&s.ct); acc.fold(&s.ss); },
                );

                // 고정 암호문으로 되돌린다
                st.ct = ct0.clone();
                st.ct_bad = ct0.clone();
                st.ct_bad[0] ^= 0x01;

                bench_util::timed(
                    "decaps", iters, &mut st,
                    |s, _| { s.ss = decaps(&s.dk, &s.ct).expect("decaps"); },
                    |s, acc| acc.fold(&s.ss),
                );

                bench_util::timed(
                    "reject", iters, &mut st,
                    |s, _| { s.ss = decaps(&s.dk, &s.ct_bad).expect("reject"); },
                    |s, acc| acc.fold(&s.ss),
                );

                Ok(())
            }

            pub fn demo(variant: &str) -> Result<(), String> {
                use rand::RngCore;
                let mut seed_buf = [0u8; 64];
                rand::thread_rng().fill_bytes(&mut seed_buf);
                let mut m = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut m);

                let (ek_b, dk_b) = keygen(&seed_buf[..SYM], &seed_buf[SYM..])?;
                let (ct, k_a) = encaps(&ek_b, &m)?;
                let k_b = decaps(&dk_b, &ct)?;

                println!("ML-KEM-{variant} 랜덤 왕복 데모 (ml-kem crate)");
                println!("  ek {} B, dk {} B, ct {} B", ek_b.len(), dk_b.len(), ct.len());
                println!("  encaps 공유키: {}", hex_encode(&k_a));
                println!("  decaps 공유키: {}", hex_encode(&k_b));
                if k_a == k_b {
                    println!("  => 일치: 정상 동작");
                    Ok(())
                } else {
                    Err("공유키 불일치!".into())
                }
            }
        }
    };
}

variant_impl!(v512, MlKem512);
variant_impl!(v768, MlKem768);
variant_impl!(v1024, MlKem1024);

/// variant 문자열에 따라 세 모듈 중 하나로 분기시키는 헬퍼.
macro_rules! dispatch {
    ($variant:expr, $f:ident ( $($arg:expr),* )) => {
        match $variant {
            "512" => v512::$f($($arg),*),
            "768" => v768::$f($($arg),*),
            "1024" => v1024::$f($($arg),*),
            other => Err(format!("알 수 없는 파라미터 세트: {other}")),
        }
    };
}

/// bool 을 돌려주는 검사 함수용 분기 (dispatch! 는 Result 전용)
fn dispatch_check(variant: &str, dk: bool, bytes: &[u8]) -> Result<bool, String> {
    Ok(match (variant, dk) {
        ("512", false) => v512::check_ek(bytes),
        ("768", false) => v768::check_ek(bytes),
        ("1024", false) => v1024::check_ek(bytes),
        ("512", true) => v512::check_dk(bytes),
        ("768", true) => v768::check_dk(bytes),
        ("1024", true) => v1024::check_dk(bytes),
        (other, _) => return Err(format!("알 수 없는 파라미터 세트: {other}")),
    })
}

fn run_tasks(variant: &str, text: &str) -> Result<String, String> {
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
                let (ek, dk) = dispatch!(variant, keygen(&a, &b))?;
                out.push_str(&format!("KEYGEN {id} {} {}\n", hex_encode(&ek), hex_encode(&dk)));
            }
            "ENCAPS" => {
                let (ct, k) = dispatch!(variant, encaps(&a, &b))?;
                out.push_str(&format!("ENCAPS {id} {} {}\n", hex_encode(&ct), hex_encode(&k)));
            }
            "DECAPS" => {
                let k = dispatch!(variant, decaps(&a, &b))?;
                // rc: ml-kem 은 실패를 반환하지 않는다(암묵적 거부). 형식을 맞추기 위해 0.
                out.push_str(&format!("DECAPS {id} {} 0\n", hex_encode(&k)));
            }
            // 키 유효성 검사: rc 0 = 통과(유효), 1 = 거부
            "CHECKEK" => {
                let rc = i32::from(!dispatch_check(variant, false, &a)?);
                out.push_str(&format!("CHECKEK {id} {rc} 0\n"));
            }
            "CHECKDK" => {
                let rc = i32::from(!dispatch_check(variant, true, &a)?);
                out.push_str(&format!("CHECKDK {id} {rc} 0\n"));
            }
            _ => return Err(format!("알 수 없는 op: {op}")),
        }
    }
    Ok(out)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <512|768|1024> <tasks.txt|--demo|--bench [iters]>", args[0]);
        return ExitCode::from(2);
    }
    let variant = args[1].as_str();

    if args[2] == "--bench" {
        let iters: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(100);
        return match dispatch!(variant, bench(variant, iters)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => { eprintln!("벤치 실패: {e}"); ExitCode::from(1) }
        };
    }

    if args[2] == "--demo" {
        return match dispatch!(variant, demo(variant)) {
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
    match run_tasks(variant, &text) {
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
