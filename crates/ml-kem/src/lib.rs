//! impl #4 — C2Rust 산출물을 관용적 Rust 로 리팩터링한 ML-KEM.
//!
//! ## 이 크레이트의 위치
//!
//! 같은 저장소의 `rust_out/*` 는 C2Rust 가 기계적으로 변환한 결과물로,
//! 원시 포인터와 `unsafe` 로 가득한 "C 를 Rust 문법으로 옮긴" 코드다.
//! 이 크레이트는 그 코드를 사람이 읽고 쓰는 Rust 로 다시 쓴 것이다.
//!
//! | 항목 | C2Rust 산출물 | 이 크레이트 |
//! |---|---|---|
//! | 메모리 접근 | `*mut u8` + 오프셋 연산 | 슬라이스 + 경계 검사 |
//! | 다항식 | `*mut int16_t` | `Poly { coeffs: [i16; 256] }` |
//! | 파라미터 세트 | 매크로로 3회 재컴파일 | `const K: usize` 제네릭 1벌 |
//! | unsafe | 거의 모든 함수 | 없음 (`forbid(unsafe_code)`) |
//! | 상수시간 배리어 | volatile 전역 XOR | 마스크 산술 |
//!
//! 알고리즘 자체(레이어 구조, zeta 순서, 비트 패킹, nonce 배정)는 동일하게 유지해
//! ACVP KAT 이 바이트 단위로 일치한다.

#![forbid(unsafe_code)]

pub mod fips202;
pub mod indcpa;
pub mod kem;
pub mod params;
pub mod poly;
pub mod polyvec;
pub mod sampling;
pub mod zetas;

pub use params::{ciphertext_bytes, public_key_bytes, secret_key_bytes, SSBYTES, SYMBYTES};

/// 파라미터 세트별 편의 API.
pub mod ml_kem {
    use crate::kem;
    use crate::params::SYMBYTES;

    macro_rules! variant {
        ($name:ident, $k:expr) => {
            /// ML-KEM 파라미터 세트별 진입점
            pub mod $name {
                use super::*;

                pub const K: usize = $k;
                pub const PUBLIC_KEY_BYTES: usize = crate::params::public_key_bytes($k);
                pub const SECRET_KEY_BYTES: usize = crate::params::secret_key_bytes($k);
                pub const CIPHERTEXT_BYTES: usize = crate::params::ciphertext_bytes($k);

                /// KeyGen_internal(d ‖ z)
                pub fn keypair_derand(pk: &mut [u8], sk: &mut [u8], coins: &[u8; 2 * SYMBYTES]) {
                    kem::keypair_derand::<$k>(pk, sk, coins)
                }
                /// Encaps_internal(ek, m)
                pub fn enc_derand(
                    ct: &mut [u8],
                    ss: &mut [u8],
                    pk: &[u8],
                    coins: &[u8; SYMBYTES],
                ) -> Result<(), kem::KeyError> {
                    kem::enc_derand::<$k>(ct, ss, pk, coins)
                }
                /// Decaps(dk, c)
                pub fn dec(ss: &mut [u8], ct: &[u8], sk: &[u8]) -> Result<(), kem::KeyError> {
                    kem::dec::<$k>(ss, ct, sk)
                }

                /// FIPS 203 7.2 모듈러스 검사 (단독 호출용)
                pub fn check_ek(pk: &[u8]) -> Result<(), kem::KeyError> {
                    kem::check_ek::<$k>(pk)
                }

                /// FIPS 203 7.3 해시 검사 (단독 호출용)
                pub fn check_dk(sk: &[u8]) -> Result<(), kem::KeyError> {
                    kem::check_dk::<$k>(sk)
                }
            }
        };
    }

    variant!(mlkem512, 2);
    variant!(mlkem768, 3);
    variant!(mlkem1024, 4);
}
