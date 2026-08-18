//! 4구현 공용 벤치 측정 유틸.
//!
//! 공정성을 위해 다음을 모든 구현에서 동일하게 맞춘다.
//!   * 입력 패턴 (`base_coins`, `base_msg`)
//!   * 워밍업 횟수 (iters/10 + 1)
//!   * 타이밍 구간 — 연산만. 체크섬 접기는 계측 구간 밖에서 수행
//!   * 체크섬 식 (순서 민감 롤링 해시)
//!
//! 출력 한 줄 형식 (C 하네스와 동일):
//!   `BENCH <op> <iters> <total_ns> <best_ns> <checksum_hex>`

use std::time::Instant;

/// keygen 용 고정 코인 d‖z (64B). C 쪽 `base_coins()` 와 같은 식.
pub fn base_coins() -> [u8; 64] {
    core::array::from_fn(|j| ((j * 7 + 1) & 0xFF) as u8)
}

/// encaps 용 고정 메시지 m (32B). C 쪽 `base_msg()` 와 같은 식.
pub fn base_msg() -> [u8; 32] {
    core::array::from_fn(|j| ((j * 11 + 3) & 0xFF) as u8)
}

/// 순서에 민감한 롤링 해시. 같은 값이 짝수 번 들어와도 상쇄되지 않는다.
#[derive(Default)]
pub struct Acc(u64);

impl Acc {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn fold(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(1_000_003).wrapping_add(u64::from(b));
        }
    }

    pub fn hex(&self) -> String {
        format!("{:016x}", self.0)
    }
}

/// 한 연산을 벤치한다.
///
/// `state` 는 출력 버퍼 묶음이다. `run` 은 연산만 수행하고, `fold` 는 계측 구간
/// 밖에서 결과 바이트를 체크섬에 접는다 — C 하네스와 동일한 구조.
pub fn timed<S, F, G>(op: &str, iters: usize, state: &mut S, mut run: F, fold: G)
where
    F: FnMut(&mut S, usize),
    G: Fn(&S, &mut Acc),
{
    let warm = iters / 10 + 1;
    for i in 0..warm {
        run(state, i);
    }

    let mut acc = Acc::new();
    let mut total_ns: u128 = 0;
    let mut best_ns = u64::MAX;

    for i in 0..iters {
        let t0 = Instant::now();
        run(state, i);
        let dt = t0.elapsed().as_nanos() as u64;

        total_ns += u128::from(dt);
        best_ns = best_ns.min(dt);

        fold(state, &mut acc);
    }

    println!(
        "BENCH {op} {iters} {total_ns} {best_ns} {}",
        acc.hex()
    );
}
