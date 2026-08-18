//! FIPS 202 (SHA3/SHAKE) 래퍼.
//!
//! C 쪽 symmetric.h 의 hash_h/hash_g/hash_j/prf/XOF 에 대응한다.
//! C2Rust 구현이 shim 을 통해 `sha3` crate 를 쓰는 것과 동일한 선택이므로,
//! 두 구현의 차이는 "ML-KEM 로직" 뿐이 된다.

use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};

/// H = SHA3-256
pub fn hash_h(out: &mut [u8; 32], input: &[u8]) {
    out.copy_from_slice(&Sha3_256::digest(input));
}

/// G = SHA3-512  (출력 64바이트를 두 개의 32바이트로 쓴다)
pub fn hash_g(out: &mut [u8; 64], input: &[u8]) {
    out.copy_from_slice(&Sha3_512::digest(input));
}

/// J = SHAKE256(·, 32) — 암묵적 거부용 공유키 유도
pub fn hash_j(out: &mut [u8; 32], input: &[u8]) {
    let mut h = Shake256::default();
    h.update(input);
    h.finalize_xof().read(out);
}

/// PRF = SHAKE256(seed ‖ nonce, eta*64)
pub fn prf(out: &mut [u8], seed: &[u8; 32], nonce: u8) {
    let mut h = Shake256::default();
    h.update(seed);
    h.update(&[nonce]);
    h.finalize_xof().read(out);
}

/// XOF = SHAKE128, 행렬 A 생성용. 블록 단위로 계속 뽑아 쓸 수 있어야 하므로
/// reader 를 그대로 돌려준다.
pub const XOF_RATE: usize = 168;

pub fn xof_absorb(seed: &[u8; 32], x: u8, y: u8) -> impl XofReader {
    let mut h = Shake128::default();
    h.update(seed);
    h.update(&[x, y]);
    h.finalize_xof()
}
