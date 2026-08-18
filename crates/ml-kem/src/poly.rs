//! 다항식 타입과 Z_q 산술, NTT.
//!
//! C2Rust 산출물에서는 `*mut int16_t` 와 포인터 연산으로 표현되던 것을
//! 여기서는 고정 크기 배열을 감싼 `Poly` 타입과 슬라이스 인덱싱으로 옮겼다.
//! 알고리즘(레이어 구조, zeta 순서, Montgomery/Barrett 상수)은 원본과 동일하다.

use crate::params::{N, Q, Q_HALF, Q_I32};
use crate::zetas::ZETAS;

/// R^-1 mod q 를 곱하는 Montgomery 축약에 쓰이는 상수.
/// 62209 == q^-1 mod 2^16
const QINV: u32 = 62209;

/// 256차 다항식. 계수는 문맥에 따라 signed 또는 canonical([0,q)) 표현.
#[derive(Clone, Copy)]
pub struct Poly {
    pub coeffs: [i16; N],
}

impl Default for Poly {
    fn default() -> Self {
        Self { coeffs: [0; N] }
    }
}

/// NTT 영역 곱셈용 사전계산 값. C 의 `mlk_poly_mulcache` 대응.
///
/// NTT 영역의 곱은 2차식 쌍 단위로 이루어지고, 그때마다 한쪽 계수에 zeta 를 곱해야 한다.
/// 같은 다항식을 여러 번 곱할 때 이 값을 미리 구해 두면 매번 다시 곱하지 않아도 된다.
#[derive(Clone, Copy)]
pub struct MulCache {
    pub coeffs: [i16; N / 2],
}

impl Default for MulCache {
    fn default() -> Self {
        Self { coeffs: [0; N / 2] }
    }
}

/// a * R^-1 mod q. |결과| <= ceil(|a|/2^16) + (q+1)/2
pub(crate) fn montgomery_reduce(a: i32) -> i16 {
    // a * q^-1 mod 2^16 (unsigned 표현으로 계산 후 signed 로 해석)
    let a_inverted = (a as u32).wrapping_mul(QINV) as u16;
    let t = a_inverted as i16;
    let r = a - (t as i32) * Q_I32;
    (r >> 16) as i16
}

/// Montgomery 곱: a*b*R^-1 mod q
pub(crate) fn fqmul(a: i16, b: i16) -> i16 {
    montgomery_reduce(a as i32 * b as i32)
}

/// a 와 합동인 중심 표현(|결과| <= q/2). magic 20159 == round(2^26 / q)
fn barrett_reduce(a: i16) -> i16 {
    const MAGIC: i32 = 20159;
    let t = (MAGIC * a as i32 + (1 << 25)) >> 26;
    (a as i32 - t * Q_I32) as i16
}

/// 값 배리어. 입력을 그대로 돌려주지만 컴파일러가 그 값의 범위를 추론하지 못하게 한다.
///
/// 상수시간 코드는 조건 분기 대신 마스크 연산으로 값을 고른다. 그런데 컴파일러가
/// "마스크는 0 아니면 전부 1이니 분기로 바꾸면 빠르다"고 판단하면 분기가 되살아나고,
/// 그 순간 실행 시간에 비밀 정보가 새어 나간다. 이를 막기 위해 원본 C 는 빈 어셈블리
/// 블록이나 항상 0인 volatile 변수와의 XOR 을 쓴다. Rust 에서 같은 역할을 하는 안전한
/// 수단이 `black_box` 로, 값을 바꾸지 않으면서 최적화기에게 불투명하게 만든다.
#[inline]
pub(crate) fn value_barrier_i16(b: i16) -> i16 {
    core::hint::black_box(b)
}

#[inline]
pub(crate) fn value_barrier_u8(b: u8) -> u8 {
    core::hint::black_box(b)
}

/// 음수를 [0,q) 로 올린다. 분기 없이(상수시간) 수행.
fn signed_to_unsigned_q(c: i16) -> i16 {
    // c < 0 이면 mask = -1, 아니면 0.
    // 배리어를 통과시켜 컴파일러가 이 마스크를 분기로 되돌리지 못하게 한다.
    let mask = value_barrier_i16(c >> 15);
    c + (mask & Q)
}

impl Poly {
    /// 모든 계수를 canonical [0,q) 로 만든다.
    pub fn reduce(&mut self) {
        for c in self.coeffs.iter_mut() {
            *c = signed_to_unsigned_q(barrett_reduce(*c));
        }
    }

    /// Montgomery 도메인으로 올린다 (× R^2 mod q, f = 2^32 mod q = 1353).
    pub fn tomont(&mut self) {
        const F: i16 = ((1i64 << 32) % Q as i64) as i16;
        for c in self.coeffs.iter_mut() {
            *c = fqmul(*c, F);
        }
    }

    pub fn add(&mut self, b: &Poly) {
        for (a, b) in self.coeffs.iter_mut().zip(b.coeffs.iter()) {
            *a = a.wrapping_add(*b);
        }
    }

    pub fn sub(&mut self, b: &Poly) {
        for (a, b) in self.coeffs.iter_mut().zip(b.coeffs.iter()) {
            *a = a.wrapping_sub(*b);
        }
    }

    /// 정방향 NTT. 레이어 1..=7, len = 256>>layer.
    pub fn ntt(&mut self) {
        let r = &mut self.coeffs;
        let mut k = 1usize;
        for layer in 1..=7u32 {
            let len = N >> layer;
            let mut start = 0;
            while start < N {
                let zeta = ZETAS[k];
                k += 1;
                for j in start..start + len {
                    let t = fqmul(r[j + len], zeta);
                    r[j + len] = r[j].wrapping_sub(t);
                    r[j] = r[j].wrapping_add(t);
                }
                start += 2 * len;
            }
        }
    }

    /// 역 NTT (결과는 Montgomery 도메인). 마지막에 f = 2^(32-7) mod q = 1441 을 곱한다.
    pub fn invntt_tomont(&mut self) {
        const F: i16 = 1441;
        let r = &mut self.coeffs;
        for layer in (1..=7u32).rev() {
            let len = N >> layer;
            let mut k = (1usize << layer) - 1;
            let mut start = 0;
            while start < N {
                let zeta = ZETAS[k];
                if k > 0 {
                    k -= 1;
                }
                for j in start..start + len {
                    let t = r[j];
                    r[j] = barrett_reduce(t.wrapping_add(r[j + len]));
                    r[j + len] = r[j + len].wrapping_sub(t);
                    r[j + len] = fqmul(r[j + len], zeta);
                }
                start += 2 * len;
            }
        }
        for c in r.iter_mut() {
            *c = fqmul(*c, F);
        }
    }

    /// 이 다항식을 여러 번 곱할 때 재사용할 사전계산 값을 만든다.
    ///
    /// C 의 `mlk_poly_mulcache_compute` 와 같은 식이다.
    pub fn mulcache(&self) -> MulCache {
        let mut x = MulCache::default();
        for i in 0..N / 4 {
            let zeta = ZETAS[64 + i];
            x.coeffs[2 * i] = fqmul(self.coeffs[4 * i + 1], zeta);
            x.coeffs[2 * i + 1] = fqmul(self.coeffs[4 * i + 3], -zeta);
        }
        x
    }

    // ---------------- 직렬화 / 압축 ----------------

    /// 12비트 계수 2개를 3바이트로. 입력은 canonical 이어야 한다.
    pub fn tobytes(&self, r: &mut [u8]) {
        for i in 0..N / 2 {
            let t0 = self.coeffs[2 * i] as u16;
            let t1 = self.coeffs[2 * i + 1] as u16;
            r[3 * i] = (t0 & 0xFF) as u8;
            r[3 * i + 1] = ((t0 >> 8) | ((t1 << 4) & 0xF0)) as u8;
            r[3 * i + 2] = (t1 >> 4) as u8;
        }
    }

    pub fn frombytes(a: &[u8]) -> Poly {
        let mut p = Poly::default();
        for i in 0..N / 2 {
            let (t0, t1, t2) = (a[3 * i] as u16, a[3 * i + 1] as u16, a[3 * i + 2] as u16);
            p.coeffs[2 * i] = (t0 | ((t1 << 8) & 0xFFF)) as i16;
            p.coeffs[2 * i + 1] = ((t1 >> 4) | (t2 << 4)) as i16;
        }
        p
    }

    /// 메시지 32바이트 → 계수(비트가 1이면 (q+1)/2). Decompress_1.
    pub fn frommsg(msg: &[u8]) -> Poly {
        let mut p = Poly::default();
        for i in 0..N / 8 {
            for j in 0..8 {
                // 비트 선택이 분기로 인식되지 않도록 배리어를 통과시킨다 (원본 C 와 동일)
                let bit = value_barrier_u8((msg[i] >> j) & 1);
                // 분기 대신 마스크로 선택 (상수시간)
                let mask = value_barrier_i16((i16::from(bit)).wrapping_neg());
                p.coeffs[8 * i + j] = mask & Q_HALF;
            }
        }
        p
    }

    /// 계수 → 메시지 비트. Compress_1.
    pub fn tomsg(&self, msg: &mut [u8]) {
        for i in 0..N / 8 {
            msg[i] = 0;
            for j in 0..8 {
                let t = compress(self.coeffs[8 * i + j], 1) as u8;
                msg[i] |= t << j;
            }
        }
    }
}

/// Compress_d(u) = round(u * 2^d / q) mod 2^d
///
/// C 원본은 magic 상수 곱셈으로 같은 값을 계산한다(계약에 명시된 등식 그대로 구현).
pub fn compress(u: i16, d: u32) -> u16 {
    let m = 1u32 << d;
    let q = Q as u32;
    (((u as u32 * m + q / 2) / q) % m) as u16
}

/// Decompress_d(y) = round(y * q / 2^d)
pub fn decompress(y: u16, d: u32) -> i16 {
    let q = Q as u32;
    (((y as u32 * q) + (1 << (d - 1))) >> d) as i16
}
