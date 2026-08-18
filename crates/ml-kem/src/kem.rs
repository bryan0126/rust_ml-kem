//! ML-KEM (IND-CCA) 계층. C 쪽 kem.c 대응.
//!
//! FIPS 203 의 내부 결정적 함수 3개를 그대로 노출한다.
//!   KeyGen_internal(d, z), Encaps_internal(ek, m), Decaps(dk, c)

use crate::fips202::{hash_g, hash_h, hash_j};
use crate::indcpa;
use crate::params::{
    ciphertext_bytes, polyvec_bytes, public_key_bytes, secret_key_bytes, SSBYTES, SYMBYTES,
};
use crate::poly::{value_barrier_u8, Poly};
use crate::polyvec::{polyvec_frombytes, polyvec_tobytes};

/// 키 유효성 검사 실패. FIPS 203 이 요구하는 입력 검사에 걸린 경우다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// ek 의 계수가 정규 범위를 벗어났다 (FIPS 203 7.2 모듈러스 검사)
    InvalidEncapsulationKey,
    /// dk 안에 저장된 H(ek) 가 실제 ek 의 해시와 다르다 (FIPS 203 7.3 해시 검사)
    InvalidDecapsulationKey,
}

/// 두 슬라이스가 다르면 0xFF, 같으면 0x00 을 반환한다 (상수시간).
fn ct_diff_mask(a: &[u8], b: &[u8]) -> u8 {
    let mut acc = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        acc |= x ^ y;
    }
    // acc != 0 이면 0xFF. 배리어로 컴파일러의 범위 추론을 막는다.
    let acc = value_barrier_u8(acc);
    let nz = (u16::from(acc).wrapping_neg()) >> 8;
    value_barrier_u8(nz as u8)
}

/// mask 가 0 이면 src 를 dst 에 복사한다 (상수시간 조건부 이동).
fn ct_cmov_zero(dst: &mut [u8], src: &[u8], mask: u8) {
    // mask==0 → sel=0xFF(복사), mask==0xFF → sel=0x00(유지)
    let sel = value_barrier_u8(!mask);
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = (*d & !sel) | (*s & sel);
    }
}

/// FIPS 203 7.2 모듈러스 검사.
///
/// ek 를 다항식으로 디코드한 뒤 정규화해서 다시 인코딩하고 원본 바이트와 비교한다.
/// 12비트로 표현되지만 q 이상인 계수를 담은 비정규 키는 재인코딩 결과가 달라지므로
/// 여기서 걸러진다. 비교는 상수시간으로 수행한다.
pub fn check_ek<const K: usize>(pk: &[u8]) -> Result<(), KeyError> {
    let pv = polyvec_bytes(K);
    // FIPS 203 7.2 타입 검사: 길이가 규격과 정확히 같아야 한다.
    // 길이가 다른 키는 내용을 보지 않고 거부한다.
    if pk.len() != public_key_bytes(K) {
        return Err(KeyError::InvalidEncapsulationKey);
    }

    let mut v: [Poly; K] = polyvec_frombytes(&pk[..pv]);
    v.iter_mut().for_each(Poly::reduce);

    let mut reencoded = vec![0u8; pv];
    polyvec_tobytes(&mut reencoded, &v);

    if ct_diff_mask(&pk[..pv], &reencoded) == 0 {
        Ok(())
    } else {
        Err(KeyError::InvalidEncapsulationKey)
    }
}

/// FIPS 203 7.3 해시 검사.
///
/// dk 는 dk_pke ‖ ek ‖ H(ek) ‖ z 형태다. 안에 들어 있는 ek 로 해시를 다시 계산해
/// 저장된 값과 같은지 상수시간으로 비교한다. 손상되거나 조작된 dk 를 걸러낸다.
pub fn check_dk<const K: usize>(sk: &[u8]) -> Result<(), KeyError> {
    let pv = polyvec_bytes(K);
    let pkb = public_key_bytes(K);
    let skb = secret_key_bytes(K);
    // FIPS 203 7.3 타입 검사: 길이가 규격과 정확히 같아야 한다.
    if sk.len() != skb {
        return Err(KeyError::InvalidDecapsulationKey);
    }

    let mut h = [0u8; 32];
    hash_h(&mut h, &sk[pv..pv + pkb]);

    if ct_diff_mask(&sk[skb - 2 * SYMBYTES..skb - SYMBYTES], &h) == 0 {
        Ok(())
    } else {
        Err(KeyError::InvalidDecapsulationKey)
    }
}

/// KeyGen_internal(d, z) → (ek, dk)
///
/// dk = dk_pke ‖ ek ‖ H(ek) ‖ z
pub fn keypair_derand<const K: usize>(pk: &mut [u8], sk: &mut [u8], coins: &[u8; 2 * SYMBYTES]) {
    let (d, z) = coins.split_at(SYMBYTES);
    let d: &[u8; 32] = d.try_into().unwrap();

    indcpa::keypair_derand::<K>(pk, sk, d);

    let pkb = public_key_bytes(K);
    let skb = secret_key_bytes(K);
    let pv = polyvec_bytes(K);

    // ek 사본
    sk[pv..pv + pkb].copy_from_slice(&pk[..pkb]);
    // H(ek)
    let mut h = [0u8; 32];
    hash_h(&mut h, &pk[..pkb]);
    sk[skb - 2 * SYMBYTES..skb - SYMBYTES].copy_from_slice(&h);
    // z (거부 시 의사난수 유도용)
    sk[skb - SYMBYTES..skb].copy_from_slice(z);
}

/// Encaps_internal(ek, m) → (c, K)
pub fn enc_derand<const K: usize>(
    ct: &mut [u8],
    ss: &mut [u8],
    pk: &[u8],
    coins: &[u8; SYMBYTES],
) -> Result<(), KeyError> {
    // FIPS 203 7.2 모듈러스 검사를 먼저 수행한다 (원본 C 와 같은 순서)
    check_ek::<K>(pk)?;

    // (K, r) = G(m ‖ H(ek))
    let mut buf = [0u8; 2 * SYMBYTES];
    buf[..SYMBYTES].copy_from_slice(coins);
    let mut h = [0u8; 32];
    hash_h(&mut h, &pk[..public_key_bytes(K)]);
    buf[SYMBYTES..].copy_from_slice(&h);

    let mut kr = [0u8; 64];
    hash_g(&mut kr, &buf);

    let m: &[u8; 32] = buf[..SYMBYTES].try_into().unwrap();
    let r: &[u8; 32] = kr[SYMBYTES..].try_into().unwrap();
    indcpa::enc::<K>(ct, m, pk, r);

    ss[..SSBYTES].copy_from_slice(&kr[..SYMBYTES]);
    Ok(())
}

/// Decaps(dk, c) → K
///
/// 재암호화 결과가 c 와 다르면 규격대로 J(z ‖ c) 를 돌려준다(암묵적 거부).
/// 오류를 반환하지 않는 것이 FIPS 203 의 정의이다.
pub fn dec<const K: usize>(ss: &mut [u8], ct: &[u8], sk: &[u8]) -> Result<(), KeyError> {
    // FIPS 203 7.3 해시 검사를 먼저 수행한다 (원본 C 와 같은 순서)
    check_dk::<K>(sk)?;

    let pv = polyvec_bytes(K);
    let pkb = public_key_bytes(K);
    let skb = secret_key_bytes(K);
    let ctb = ciphertext_bytes(K);
    let pk = &sk[pv..pv + pkb];

    // m' = Decrypt(dk_pke, c)
    let mut buf = [0u8; 2 * SYMBYTES];
    let mut m = [0u8; SYMBYTES];
    indcpa::dec::<K>(&mut m, ct, &sk[..pv]);
    buf[..SYMBYTES].copy_from_slice(&m);
    // H(ek) 는 dk 안에 저장된 것을 쓴다
    buf[SYMBYTES..].copy_from_slice(&sk[skb - 2 * SYMBYTES..skb - SYMBYTES]);

    // (K', r') = G(m' ‖ H(ek))
    let mut kr = [0u8; 64];
    hash_g(&mut kr, &buf);

    // 재암호화
    let mut tmp = vec![0u8; ctb];
    let m2: &[u8; 32] = buf[..SYMBYTES].try_into().unwrap();
    let r: &[u8; 32] = kr[SYMBYTES..].try_into().unwrap();
    indcpa::enc::<K>(&mut tmp, m2, pk, r);

    let fail = ct_diff_mask(&ct[..ctb], &tmp);

    // 거부 시 공유키: J(z ‖ c)
    let mut zc = Vec::with_capacity(SYMBYTES + ctb);
    zc.extend_from_slice(&sk[skb - SYMBYTES..skb]);
    zc.extend_from_slice(&ct[..ctb]);
    let mut kbar = [0u8; 32];
    hash_j(&mut kbar, &zc);

    ss[..SSBYTES].copy_from_slice(&kbar);
    // 성공(fail==0)이면 K' 로 덮어쓴다
    ct_cmov_zero(&mut ss[..SSBYTES], &kr[..SYMBYTES], fail);
    Ok(())
}
