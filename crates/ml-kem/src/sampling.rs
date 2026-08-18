//! 샘플링: 균등분포 거부표집(행렬 A)과 중심이항분포(노이즈).
//!
//! C 쪽 sampling.c 대응. C2Rust 산출물의 포인터/버퍼 오프셋 연산을
//! 슬라이스와 이터레이터로 옮겼다.

use sha3::digest::XofReader;

use crate::fips202::{prf, xof_absorb, XOF_RATE};
use crate::params::{eta1, ETA2, N, Q};
use crate::poly::Poly;

/// 12비트씩 잘라 q 미만인 값만 채택한다. 채운 개수를 반환.
fn rej_uniform(coeffs: &mut [i16], mut ctr: usize, buf: &[u8]) -> usize {
    let target = coeffs.len();
    let mut pos = 0;
    while ctr < target && pos + 3 <= buf.len() {
        let val0 = (u16::from(buf[pos]) | (u16::from(buf[pos + 1]) << 8)) & 0xFFF;
        let val1 = ((u16::from(buf[pos + 1]) >> 4) | (u16::from(buf[pos + 2]) << 4)) & 0xFFF;
        pos += 3;

        if (val0 as i16) < Q {
            coeffs[ctr] = val0 as i16;
            ctr += 1;
        }
        if ctr < target && (val1 as i16) < Q {
            coeffs[ctr] = val1 as i16;
            ctr += 1;
        }
    }
    ctr
}

/// A 행렬의 한 원소를 seed‖(x,y) 로부터 생성한다.
fn sample_matrix_entry(seed: &[u8; 32], x: u8, y: u8) -> Poly {
    // C 의 MLKEM_GEN_MATRIX_NBLOCKS 와 동일한 초기 블록 수
    const NBLOCKS: usize = (12 * N / 8 * (1 << 12) / Q as usize + XOF_RATE) / XOF_RATE;

    let mut reader = xof_absorb(seed, x, y);
    let mut p = Poly::default();

    let mut buf = vec![0u8; NBLOCKS * XOF_RATE];
    reader.read(&mut buf);
    let mut ctr = rej_uniform(&mut p.coeffs, 0, &buf);

    // 부족하면 한 블록씩 추가로 짜낸다 (C 와 동일한 전략)
    let mut block = [0u8; XOF_RATE];
    while ctr < N {
        reader.read(&mut block);
        ctr = rej_uniform(&mut p.coeffs, ctr, &block);
    }
    p
}

/// 행렬 A (또는 A^T) 를 생성한다.
///
/// C: `x = i / K`, `y = i % K`, seed 접미는 transposed ? (x,y) : (y,x).
/// 즉 a[x][y] 는 transposed 일 때 seed‖(x,y) 로 생성된다.
pub fn gen_matrix<const K: usize>(seed: &[u8; 32], transposed: bool) -> [[Poly; K]; K] {
    let mut a = [[Poly::default(); K]; K];
    for x in 0..K {
        for y in 0..K {
            let (b0, b1) = if transposed {
                (x as u8, y as u8)
            } else {
                (y as u8, x as u8)
            };
            a[x][y] = sample_matrix_entry(seed, b0, b1);
        }
    }
    a
}

/// CBD_2: 4바이트에서 8계수
fn cbd2(buf: &[u8]) -> Poly {
    let mut p = Poly::default();
    for i in 0..N / 8 {
        let t = u32::from_le_bytes([buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]]);
        let mut d = t & 0x5555_5555;
        d += (t >> 1) & 0x5555_5555;

        for j in 0..8 {
            let a = ((d >> (4 * j)) & 0x3) as i16;
            let b = ((d >> (4 * j + 2)) & 0x3) as i16;
            p.coeffs[8 * i + j] = a - b;
        }
    }
    p
}

/// CBD_3: 3바이트에서 4계수
fn cbd3(buf: &[u8]) -> Poly {
    let mut p = Poly::default();
    for i in 0..N / 4 {
        let t = u32::from(buf[3 * i])
            | (u32::from(buf[3 * i + 1]) << 8)
            | (u32::from(buf[3 * i + 2]) << 16);
        let mut d = t & 0x0024_9249;
        d += (t >> 1) & 0x0024_9249;
        d += (t >> 2) & 0x0024_9249;

        for j in 0..4 {
            let a = ((d >> (6 * j)) & 0x7) as i16;
            let b = ((d >> (6 * j + 3)) & 0x7) as i16;
            p.coeffs[4 * i + j] = a - b;
        }
    }
    p
}

fn cbd(buf: &[u8], eta: usize) -> Poly {
    match eta {
        2 => cbd2(buf),
        3 => cbd3(buf),
        _ => unreachable!("ML-KEM 은 eta ∈ {{2,3}} 만 사용한다"),
    }
}

/// PRF(seed, nonce) → CBD_eta 노이즈 다항식
fn getnoise(seed: &[u8; 32], nonce: u8, eta: usize) -> Poly {
    let mut buf = [0u8; 3 * N / 4]; // eta=3 최대 크기(192B)
    let len = eta * N / 4;
    prf(&mut buf[..len], seed, nonce);
    cbd(&buf[..len], eta)
}

pub fn getnoise_eta1<const K: usize>(seed: &[u8; 32], nonce: u8) -> Poly {
    getnoise(seed, nonce, eta1(K))
}

pub fn getnoise_eta2(seed: &[u8; 32], nonce: u8) -> Poly {
    getnoise(seed, nonce, ETA2)
}
