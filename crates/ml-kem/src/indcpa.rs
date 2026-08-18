//! IND-CPA 계층 (K-PKE). C 쪽 indcpa.c 대응.

use crate::fips202::hash_g;
use crate::params::{
    ciphertext_bytes, polyvec_bytes, polyvec_compressed_du, public_key_bytes, POLYBYTES, SYMBYTES,
};
use crate::poly::Poly;
use crate::polyvec::{
    poly_compress_dv, poly_decompress_dv, polyvec_basemul_acc_cached, polyvec_compress_du,
    polyvec_decompress_du, polyvec_frombytes, polyvec_mulcache, polyvec_tobytes,
};
use crate::sampling::{gen_matrix, getnoise_eta1, getnoise_eta2};

fn polyvec_ntt<const K: usize>(v: &mut [Poly; K]) {
    v.iter_mut().for_each(Poly::ntt);
}

fn polyvec_reduce<const K: usize>(v: &mut [Poly; K]) {
    v.iter_mut().for_each(Poly::reduce);
}

/// out[i] = Σ_j a[i][j] ∘ v[j]   (NTT 영역)
///
/// v 의 사전계산 값을 K 개 행에서 그대로 재사용한다. 행마다 다시 계산하면
/// 같은 값을 K 번 구하게 된다.
fn matvec_mul<const K: usize>(
    a: &[[Poly; K]; K],
    v: &[Poly; K],
    v_cache: &[crate::poly::MulCache; K],
) -> [Poly; K] {
    core::array::from_fn(|i| polyvec_basemul_acc_cached(&a[i], v, v_cache))
}

/// K-PKE.KeyGen. coins = d (32B)
pub fn keypair_derand<const K: usize>(pk: &mut [u8], sk: &mut [u8], coins: &[u8; SYMBYTES]) {
    // (rho, sigma) = G(d ‖ K)
    let mut with_ds = [0u8; SYMBYTES + 1];
    with_ds[..SYMBYTES].copy_from_slice(coins);
    with_ds[SYMBYTES] = K as u8;

    let mut buf = [0u8; 64];
    hash_g(&mut buf, &with_ds);
    let (publicseed, noiseseed) = buf.split_at(SYMBYTES);
    let publicseed: &[u8; 32] = publicseed.try_into().unwrap();
    let noiseseed: &[u8; 32] = noiseseed.try_into().unwrap();

    let a = gen_matrix::<K>(publicseed, false);

    // s, e ← CBD_eta1, nonce 는 0..2K-1
    let mut skpv: [Poly; K] = core::array::from_fn(|i| getnoise_eta1::<K>(noiseseed, i as u8));
    let mut e: [Poly; K] = core::array::from_fn(|i| getnoise_eta1::<K>(noiseseed, (K + i) as u8));

    polyvec_ntt(&mut skpv);
    polyvec_ntt(&mut e);

    // s 의 사전계산 값을 한 번 만들어 K 개 행에서 재사용한다
    let skpv_cache = polyvec_mulcache(&skpv);

    // t = A∘s + e  (basemul 결과는 R^-1 이 붙어 있으므로 tomont 로 되돌린다)
    let mut pkpv = matvec_mul(&a, &skpv, &skpv_cache);
    for (p, ee) in pkpv.iter_mut().zip(e.iter()) {
        p.tomont();
        p.add(ee);
    }
    polyvec_reduce(&mut pkpv);
    polyvec_reduce(&mut skpv);

    polyvec_tobytes(&mut sk[..polyvec_bytes(K)], &skpv);
    polyvec_tobytes(&mut pk[..polyvec_bytes(K)], &pkpv);
    pk[polyvec_bytes(K)..public_key_bytes(K)].copy_from_slice(publicseed);
}

/// K-PKE.Encrypt. coins = r (32B)
pub fn enc<const K: usize>(
    c: &mut [u8],
    m: &[u8; SYMBYTES],
    pk: &[u8],
    coins: &[u8; SYMBYTES],
) {
    let pkpv: [Poly; K] = polyvec_frombytes(&pk[..polyvec_bytes(K)]);
    let seed: &[u8; 32] = pk[polyvec_bytes(K)..public_key_bytes(K)].try_into().unwrap();
    let k_poly = Poly::frommsg(m);

    let at = gen_matrix::<K>(seed, true);

    // r ← CBD_eta1 (nonce 0..K-1), e1 ← CBD_eta2 (nonce K..2K-1), e2 ← CBD_eta2 (nonce 2K)
    let mut sp: [Poly; K] = core::array::from_fn(|i| getnoise_eta1::<K>(coins, i as u8));
    let ep: [Poly; K] = core::array::from_fn(|i| getnoise_eta2(coins, (K + i) as u8));
    let epp = getnoise_eta2(coins, (2 * K) as u8);

    polyvec_ntt(&mut sp);

    // r 의 사전계산 값을 K 개 행 + 아래 내적까지 총 K+1 번 재사용한다
    let sp_cache = polyvec_mulcache(&sp);

    // u = invNTT(A^T ∘ r) + e1
    let mut b = matvec_mul(&at, &sp, &sp_cache);
    // v = invNTT(t^T ∘ r) + e2 + Decompress_1(m)
    let mut v = polyvec_basemul_acc_cached(&pkpv, &sp, &sp_cache);

    b.iter_mut().for_each(Poly::invntt_tomont);
    v.invntt_tomont();

    for (p, e) in b.iter_mut().zip(ep.iter()) {
        p.add(e);
    }
    v.add(&epp);
    v.add(&k_poly);

    polyvec_reduce(&mut b);
    v.reduce();

    let split = polyvec_compressed_du(K);
    polyvec_compress_du(&mut c[..split], &b);
    poly_compress_dv::<K>(&mut c[split..ciphertext_bytes(K)], &v);
}

/// K-PKE.Decrypt
pub fn dec<const K: usize>(m: &mut [u8; SYMBYTES], c: &[u8], sk: &[u8]) {
    let split = polyvec_compressed_du(K);
    let mut b: [Poly; K] = polyvec_decompress_du(&c[..split]);
    let mut v = poly_decompress_dv::<K>(&c[split..ciphertext_bytes(K)]);

    let skpv: [Poly; K] = polyvec_frombytes(&sk[..K * POLYBYTES]);

    polyvec_ntt(&mut b);
    let b_cache = polyvec_mulcache(&b);
    let mut sb = polyvec_basemul_acc_cached(&skpv, &b, &b_cache);
    sb.invntt_tomont();

    v.sub(&sb);
    v.reduce();
    v.tomsg(m);
}
