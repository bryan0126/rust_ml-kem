//! C2Rust 로 변환된 liboqs ML-KEM 코드가 링크 시 요구하는 외부 심볼 구현.
//!
//! 변환 대상에서 제외한 것은 대칭 프리미티브(FIPS202)와 RNG 뿐이므로,
//! 여기서 순수 Rust crate(sha3 / rand)로 그 심볼들만 채운다.
//!
//! 시그니처는 rust_out/*/src/*.rs 에 c2rust 가 생성해 둔 `extern "C"` 선언과
//! 정확히 일치해야 한다 (ctx 는 `{ void *ctx; }` 불투명 포인터).

use core::ffi::c_void;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake128Reader, Shake256};

// ---------------------------------------------------------------------------
// 불투명 컨텍스트: C 측 `typedef struct { void *ctx; }` 와 레이아웃 동일
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct OQS_SHA3_shake128_inc_ctx {
    pub ctx: *mut c_void,
}

#[repr(C)]
pub struct OQS_SHA3_shake128_x4_inc_ctx {
    pub ctx: *mut c_void,
}

/// absorb 단계와 squeeze 단계를 함께 담는다.
/// squeeze 는 여러 번 호출되며 스트림이 이어져야 하므로 reader 를 보존한다.
enum ShakeState {
    Absorbing(Shake128),
    Squeezing(Shake128Reader),
}

impl ShakeState {
    fn new() -> Self {
        ShakeState::Absorbing(Shake128::default())
    }

    fn absorb(&mut self, data: &[u8]) {
        match self {
            ShakeState::Absorbing(h) => h.update(data),
            ShakeState::Squeezing(_) => panic!("shake128: absorb after squeeze"),
        }
    }

    /// absorb 단계였다면 finalize 해서 squeeze 단계로 전환한 뒤 읽는다.
    fn squeeze(&mut self, out: &mut [u8]) {
        if let ShakeState::Absorbing(h) = self {
            let reader = core::mem::replace(h, Shake128::default()).finalize_xof();
            *self = ShakeState::Squeezing(reader);
        }
        match self {
            ShakeState::Squeezing(r) => r.read(out),
            ShakeState::Absorbing(_) => unreachable!(),
        }
    }
}

// ---------------------------------------------------------------------------
// SHAKE128 증분 API (단일 레인)
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_inc_init(state: *mut OQS_SHA3_shake128_inc_ctx) {
    (*state).ctx = Box::into_raw(Box::new(ShakeState::new())) as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_absorb_once(
    state: *mut OQS_SHA3_shake128_inc_ctx,
    in_0: *const u8,
    inlen: usize,
) {
    let s = &mut *((*state).ctx as *mut ShakeState);
    s.absorb(core::slice::from_raw_parts(in_0, inlen));
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_inc_squeeze(
    output: *mut u8,
    outlen: usize,
    state: *mut OQS_SHA3_shake128_inc_ctx,
) {
    let s = &mut *((*state).ctx as *mut ShakeState);
    s.squeeze(core::slice::from_raw_parts_mut(output, outlen));
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_inc_ctx_release(state: *mut OQS_SHA3_shake128_inc_ctx) {
    if !(*state).ctx.is_null() {
        drop(Box::from_raw((*state).ctx as *mut ShakeState));
        (*state).ctx = core::ptr::null_mut();
    }
}

// ---------------------------------------------------------------------------
// SHAKE128 증분 API (4-way 배치)
// sha3 crate 에는 배치 API 가 없으므로 4개 독립 상태를 순차 처리한다.
// 결과 바이트는 배치 구현과 동일하다(성능만 다름).
// ---------------------------------------------------------------------------

struct ShakeX4 {
    lanes: [ShakeState; 4],
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_x4_inc_init(state: *mut OQS_SHA3_shake128_x4_inc_ctx) {
    let b = Box::new(ShakeX4 {
        lanes: [
            ShakeState::new(),
            ShakeState::new(),
            ShakeState::new(),
            ShakeState::new(),
        ],
    });
    (*state).ctx = Box::into_raw(b) as *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_x4_absorb_once(
    state: *mut OQS_SHA3_shake128_x4_inc_ctx,
    in0: *const u8,
    in1: *const u8,
    in2: *const u8,
    in3: *const u8,
    inlen: usize,
) {
    let s = &mut *((*state).ctx as *mut ShakeX4);
    for (lane, ptr) in s.lanes.iter_mut().zip([in0, in1, in2, in3].iter()) {
        lane.absorb(core::slice::from_raw_parts(*ptr, inlen));
    }
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_x4_inc_squeeze(
    out0: *mut u8,
    out1: *mut u8,
    out2: *mut u8,
    out3: *mut u8,
    outlen: usize,
    state: *mut OQS_SHA3_shake128_x4_inc_ctx,
) {
    let s = &mut *((*state).ctx as *mut ShakeX4);
    for (lane, ptr) in s.lanes.iter_mut().zip([out0, out1, out2, out3].iter()) {
        lane.squeeze(core::slice::from_raw_parts_mut(*ptr, outlen));
    }
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake128_x4_inc_ctx_release(
    state: *mut OQS_SHA3_shake128_x4_inc_ctx,
) {
    if !(*state).ctx.is_null() {
        drop(Box::from_raw((*state).ctx as *mut ShakeX4));
        (*state).ctx = core::ptr::null_mut();
    }
}

// ---------------------------------------------------------------------------
// 일회성(one-shot) 해시
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_sha3_256(output: *mut u8, input: *const u8, inplen: usize) {
    let d = Sha3_256::digest(core::slice::from_raw_parts(input, inplen));
    core::ptr::copy_nonoverlapping(d.as_ptr(), output, 32);
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_sha3_512(output: *mut u8, input: *const u8, inplen: usize) {
    let d = Sha3_512::digest(core::slice::from_raw_parts(input, inplen));
    core::ptr::copy_nonoverlapping(d.as_ptr(), output, 64);
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake256(
    output: *mut u8,
    outlen: usize,
    input: *const u8,
    inplen: usize,
) {
    let mut h = Shake256::default();
    h.update(core::slice::from_raw_parts(input, inplen));
    h.finalize_xof()
        .read(core::slice::from_raw_parts_mut(output, outlen));
}

#[no_mangle]
pub unsafe extern "C" fn OQS_SHA3_shake256_x4(
    out0: *mut u8,
    out1: *mut u8,
    out2: *mut u8,
    out3: *mut u8,
    outlen: usize,
    in0: *const u8,
    in1: *const u8,
    in2: *const u8,
    in3: *const u8,
    inlen: usize,
) {
    let ins = [in0, in1, in2, in3];
    let outs = [out0, out1, out2, out3];
    for i in 0..4 {
        OQS_SHA3_shake256(outs[i], outlen, ins[i], inlen);
    }
}

// ---------------------------------------------------------------------------
// RNG
//
// FIPS 203 의 *_internal (derand) 함수는 난수를 인자로 직접 받으므로
// KAT 검증 경로에서는 이 함수가 호출되지 않는다.
// 랜덤 왕복 데모(keypair/enc)에서만 사용된다.
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn OQS_randombytes(random_array: *mut u8, bytes_to_read: usize) {
    use rand::RngCore;
    let buf = core::slice::from_raw_parts_mut(random_array, bytes_to_read);
    rand::thread_rng().fill_bytes(buf);
}
