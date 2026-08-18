//! 다항식 벡터의 직렬화/압축.
//!
//! C 쪽 poly_k.c / compress.c 의 polyvec_* 및 poly_compress_d{4,5,10,11} 대응.
//! 비트 패킹 레이아웃은 원본과 정확히 동일하다.

use crate::params::{du, dv, POLYBYTES, N};
use crate::poly::{compress, decompress, montgomery_reduce, MulCache, Poly};

/// 벡터 각 성분의 사전계산 값을 한 번에 만든다. C 의 `mlk_polyvec_mulcache_compute` 대응.
pub fn polyvec_mulcache<const K: usize>(v: &[Poly; K]) -> [MulCache; K] {
    core::array::from_fn(|i| v[i].mulcache())
}

/// NTT 영역 내적: r = Σ_k a[k] ∘ b[k]
///
/// C 의 `mlk_polyvec_basemul_acc_montgomery_cached` 와 같은 구조를 따른다.
/// 두 가지가 핵심이다.
///   1. `b` 의 사전계산 값(`b_cache`)을 받아 zeta 곱을 매번 다시 하지 않는다.
///   2. K 개 항을 32비트 누산기에 곱셈만으로 모두 더한 뒤, 마지막에 한 번만 축약한다.
///      곱셈마다 축약하면 계수 두 개당 축약이 5K 번 필요하지만 이 방식은 2번이면 된다.
///
/// 누산기 범위: `a` 의 계수는 12비트 미만(4096), `b` 는 i16 이므로
/// |t| <= K * 2 * 4096 * 32768 = K * 2^28 이다. K <= 4 에서 2^30 이므로 i32 에 들어간다.
pub fn polyvec_basemul_acc_cached<const K: usize>(
    a: &[Poly; K],
    b: &[Poly; K],
    b_cache: &[MulCache; K],
) -> Poly {
    let mut r = Poly::default();

    for i in 0..N / 2 {
        let mut t = [0i32; 2];
        for k in 0..K {
            let (a0, a1) = (i32::from(a[k].coeffs[2 * i]), i32::from(a[k].coeffs[2 * i + 1]));
            let (b0, b1) = (i32::from(b[k].coeffs[2 * i]), i32::from(b[k].coeffs[2 * i + 1]));
            // 2차식 쌍의 곱: (a0 + a1 X)(b0 + b1 X) mod (X^2 - zeta)
            // a1*b1*zeta 항이 사전계산 값으로 대체된다.
            t[0] += a1 * i32::from(b_cache[k].coeffs[i]);
            t[0] += a0 * b0;
            t[1] += a0 * b1;
            t[1] += a1 * b0;
        }
        r.coeffs[2 * i] = montgomery_reduce(t[0]);
        r.coeffs[2 * i + 1] = montgomery_reduce(t[1]);
    }
    r
}

pub fn polyvec_tobytes<const K: usize>(r: &mut [u8], v: &[Poly; K]) {
    for (i, p) in v.iter().enumerate() {
        p.tobytes(&mut r[i * POLYBYTES..(i + 1) * POLYBYTES]);
    }
}

pub fn polyvec_frombytes<const K: usize>(a: &[u8]) -> [Poly; K] {
    let mut v = [Poly::default(); K];
    for (i, p) in v.iter_mut().enumerate() {
        *p = Poly::frombytes(&a[i * POLYBYTES..(i + 1) * POLYBYTES]);
    }
    v
}

// ---------------- d=4 (128B) ----------------

fn compress_d4(r: &mut [u8], p: &Poly) {
    for i in 0..N / 8 {
        let t: [u8; 8] = core::array::from_fn(|j| compress(p.coeffs[8 * i + j], 4) as u8);
        r[i * 4] = t[0] | (t[1] << 4);
        r[i * 4 + 1] = t[2] | (t[3] << 4);
        r[i * 4 + 2] = t[4] | (t[5] << 4);
        r[i * 4 + 3] = t[6] | (t[7] << 4);
    }
}

fn decompress_d4(a: &[u8]) -> Poly {
    let mut p = Poly::default();
    for i in 0..N / 2 {
        p.coeffs[2 * i] = decompress(u16::from(a[i] & 0xF), 4);
        p.coeffs[2 * i + 1] = decompress(u16::from(a[i] >> 4), 4);
    }
    p
}

// ---------------- d=5 (160B) ----------------

fn compress_d5(r: &mut [u8], p: &Poly) {
    for i in 0..N / 8 {
        let t: [u16; 8] = core::array::from_fn(|j| compress(p.coeffs[8 * i + j], 5));
        r[i * 5] = (t[0] | (t[1] << 5)) as u8;
        r[i * 5 + 1] = ((t[1] >> 3) | (t[2] << 2) | (t[3] << 7)) as u8;
        r[i * 5 + 2] = ((t[3] >> 1) | (t[4] << 4)) as u8;
        r[i * 5 + 3] = ((t[4] >> 4) | (t[5] << 1) | (t[6] << 6)) as u8;
        r[i * 5 + 4] = ((t[6] >> 2) | (t[7] << 3)) as u8;
    }
}

fn decompress_d5(a: &[u8]) -> Poly {
    let mut p = Poly::default();
    for i in 0..N / 8 {
        let b = &a[i * 5..i * 5 + 5];
        let t = [
            b[0] & 0x1F,
            ((b[0] >> 5) | (b[1] << 3)) & 0x1F,
            (b[1] >> 2) & 0x1F,
            ((b[1] >> 7) | (b[2] << 1)) & 0x1F,
            ((b[2] >> 4) | (b[3] << 4)) & 0x1F,
            (b[3] >> 1) & 0x1F,
            ((b[3] >> 6) | (b[4] << 2)) & 0x1F,
            (b[4] >> 3) & 0x1F,
        ];
        for j in 0..8 {
            p.coeffs[8 * i + j] = decompress(u16::from(t[j]), 5);
        }
    }
    p
}

// ---------------- d=10 (320B) ----------------

fn compress_d10(r: &mut [u8], p: &Poly) {
    for j in 0..N / 4 {
        let t: [u16; 4] = core::array::from_fn(|k| compress(p.coeffs[4 * j + k], 10));
        r[5 * j] = (t[0] & 0xFF) as u8;
        r[5 * j + 1] = ((t[0] >> 8) | ((t[1] << 2) & 0xFF)) as u8;
        r[5 * j + 2] = ((t[1] >> 6) | ((t[2] << 4) & 0xFF)) as u8;
        r[5 * j + 3] = ((t[2] >> 4) | ((t[3] << 6) & 0xFF)) as u8;
        r[5 * j + 4] = (t[3] >> 2) as u8;
    }
}

fn decompress_d10(a: &[u8]) -> Poly {
    let mut p = Poly::default();
    for j in 0..N / 4 {
        let b = &a[5 * j..5 * j + 5];
        let t = [
            (u16::from(b[0]) | (u16::from(b[1]) << 8)) & 0x3FF,
            ((u16::from(b[1]) >> 2) | (u16::from(b[2]) << 6)) & 0x3FF,
            ((u16::from(b[2]) >> 4) | (u16::from(b[3]) << 4)) & 0x3FF,
            ((u16::from(b[3]) >> 6) | (u16::from(b[4]) << 2)) & 0x3FF,
        ];
        for k in 0..4 {
            p.coeffs[4 * j + k] = decompress(t[k], 10);
        }
    }
    p
}

// ---------------- d=11 (352B) ----------------

fn compress_d11(r: &mut [u8], p: &Poly) {
    for j in 0..N / 8 {
        let t: [u16; 8] = core::array::from_fn(|k| compress(p.coeffs[8 * j + k], 11));
        r[11 * j] = (t[0] & 0xFF) as u8;
        r[11 * j + 1] = ((t[0] >> 8) | ((t[1] << 3) & 0xFF)) as u8;
        r[11 * j + 2] = ((t[1] >> 5) | ((t[2] << 6) & 0xFF)) as u8;
        r[11 * j + 3] = ((t[2] >> 2) & 0xFF) as u8;
        r[11 * j + 4] = ((t[2] >> 10) | ((t[3] << 1) & 0xFF)) as u8;
        r[11 * j + 5] = ((t[3] >> 7) | ((t[4] << 4) & 0xFF)) as u8;
        r[11 * j + 6] = ((t[4] >> 4) | ((t[5] << 7) & 0xFF)) as u8;
        r[11 * j + 7] = ((t[5] >> 1) & 0xFF) as u8;
        r[11 * j + 8] = ((t[5] >> 9) | ((t[6] << 2) & 0xFF)) as u8;
        r[11 * j + 9] = ((t[6] >> 6) | ((t[7] << 5) & 0xFF)) as u8;
        r[11 * j + 10] = (t[7] >> 3) as u8;
    }
}

fn decompress_d11(a: &[u8]) -> Poly {
    let mut p = Poly::default();
    for j in 0..N / 8 {
        let b = &a[11 * j..11 * j + 11];
        let g = |i: usize| u16::from(b[i]);
        let t = [
            (g(0) | (g(1) << 8)) & 0x7FF,
            ((g(1) >> 3) | (g(2) << 5)) & 0x7FF,
            ((g(2) >> 6) | (g(3) << 2) | (g(4) << 10)) & 0x7FF,
            ((g(4) >> 1) | (g(5) << 7)) & 0x7FF,
            ((g(5) >> 4) | (g(6) << 4)) & 0x7FF,
            ((g(6) >> 7) | (g(7) << 1) | (g(8) << 9)) & 0x7FF,
            ((g(8) >> 2) | (g(9) << 6)) & 0x7FF,
            ((g(9) >> 5) | (g(10) << 3)) & 0x7FF,
        ];
        for k in 0..8 {
            p.coeffs[8 * j + k] = decompress(t[k], 11);
        }
    }
    p
}

// ---------------- 파라미터에 따른 분기 ----------------

/// u 벡터 압축 (K 에 따라 d=10 또는 11)
pub fn polyvec_compress_du<const K: usize>(r: &mut [u8], v: &[Poly; K]) {
    let stride = 32 * du(K);
    for (i, p) in v.iter().enumerate() {
        let chunk = &mut r[i * stride..(i + 1) * stride];
        if du(K) == 10 {
            compress_d10(chunk, p);
        } else {
            compress_d11(chunk, p);
        }
    }
}

pub fn polyvec_decompress_du<const K: usize>(a: &[u8]) -> [Poly; K] {
    let stride = 32 * du(K);
    let mut v = [Poly::default(); K];
    for (i, p) in v.iter_mut().enumerate() {
        let chunk = &a[i * stride..(i + 1) * stride];
        *p = if du(K) == 10 {
            decompress_d10(chunk)
        } else {
            decompress_d11(chunk)
        };
    }
    v
}

/// v 스칼라 압축 (K 에 따라 d=4 또는 5)
pub fn poly_compress_dv<const K: usize>(r: &mut [u8], p: &Poly) {
    if dv(K) == 4 {
        compress_d4(r, p);
    } else {
        compress_d5(r, p);
    }
}

pub fn poly_decompress_dv<const K: usize>(a: &[u8]) -> Poly {
    if dv(K) == 4 {
        decompress_d4(a)
    } else {
        decompress_d5(a)
    }
}
