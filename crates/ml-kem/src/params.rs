//! ML-KEM 파라미터 (C 쪽 params.h 대응).
//!
//! C 는 `MLK_CONFIG_PARAMETER_SET` 매크로로 파일 전체를 세 번 재컴파일하지만,
//! 여기서는 K 를 const generic 으로 두어 한 벌의 코드로 세 파라미터 세트를 만든다.
//! ETA1/DU/DV 는 K 로부터 유일하게 결정되므로 const fn 으로 유도한다.

pub const N: usize = 256;
pub const Q: i16 = 3329;
pub const Q_I32: i32 = Q as i32;
/// (Q+1)/2 = 1665 — Decompress_1(1)
pub const Q_HALF: i16 = (Q + 1) / 2;
pub const SYMBYTES: usize = 32;
pub const SSBYTES: usize = 32;
pub const POLYBYTES: usize = 384;

/// 노이즈 분포 파라미터 η1 (K=2 만 3, 나머지는 2)
pub const fn eta1(k: usize) -> usize {
    if k == 2 { 3 } else { 2 }
}
/// η2 는 모든 세트에서 2
pub const ETA2: usize = 2;
/// u(벡터) 압축 비트수
pub const fn du(k: usize) -> usize {
    if k == 4 { 11 } else { 10 }
}
/// v(스칼라) 압축 비트수
pub const fn dv(k: usize) -> usize {
    if k == 4 { 5 } else { 4 }
}

pub const fn polyvec_bytes(k: usize) -> usize { k * POLYBYTES }
/// du 비트 × 256계수 = 32*du 바이트
pub const fn poly_compressed_du(k: usize) -> usize { 32 * du(k) }
pub const fn poly_compressed_dv(k: usize) -> usize { 32 * dv(k) }
pub const fn polyvec_compressed_du(k: usize) -> usize { k * poly_compressed_du(k) }

/// ek = t^ ‖ rho
pub const fn public_key_bytes(k: usize) -> usize { polyvec_bytes(k) + SYMBYTES }
/// dk = dk_pke ‖ ek ‖ H(ek) ‖ z
pub const fn secret_key_bytes(k: usize) -> usize {
    polyvec_bytes(k) + public_key_bytes(k) + 2 * SYMBYTES
}
/// c = u_compressed ‖ v_compressed
pub const fn ciphertext_bytes(k: usize) -> usize {
    polyvec_compressed_du(k) + poly_compressed_dv(k)
}
