extern "C" {
    fn OQS_SHA3_shake128_inc_init(state: *mut OQS_SHA3_shake128_inc_ctx);
    fn OQS_SHA3_shake128_inc_squeeze(
        output: *mut uint8_t,
        outlen: size_t,
        state: *mut OQS_SHA3_shake128_inc_ctx,
    );
    fn OQS_SHA3_shake128_inc_ctx_release(state: *mut OQS_SHA3_shake128_inc_ctx);
    fn OQS_SHA3_shake128_absorb_once(
        state: *mut OQS_SHA3_shake128_inc_ctx,
        in_0: *const uint8_t,
        inlen: size_t,
    );
    fn OQS_SHA3_shake128_x4_inc_init(state: *mut OQS_SHA3_shake128_x4_inc_ctx);
    fn OQS_SHA3_shake128_x4_inc_squeeze(
        out0: *mut uint8_t,
        out1: *mut uint8_t,
        out2: *mut uint8_t,
        out3: *mut uint8_t,
        outlen: size_t,
        state: *mut OQS_SHA3_shake128_x4_inc_ctx,
    );
    fn OQS_SHA3_shake128_x4_inc_ctx_release(state: *mut OQS_SHA3_shake128_x4_inc_ctx);
    fn OQS_SHA3_shake128_x4_absorb_once(
        state: *mut OQS_SHA3_shake128_x4_inc_ctx,
        in0: *const uint8_t,
        in1: *const uint8_t,
        in2: *const uint8_t,
        in3: *const uint8_t,
        inlen: size_t,
    );
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_poly(pub mlk_poly_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_poly_Inner {
    pub coeffs: [int16_t; 256],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_poly_PADDING: usize = ::core::mem::size_of::<mlk_poly>()
    - ::core::mem::size_of::<mlk_poly_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OQS_SHA3_shake128_x4_inc_ctx {
    pub ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OQS_SHA3_shake128_inc_ctx {
    pub ctx: *mut ::core::ffi::c_void,
}
pub const MLKEM_N: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MLKEM_Q: ::core::ffi::c_int = 3329 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mlk_zeroize(mut ptr: *mut ::core::ffi::c_void, mut len: size_t) {
    let mut volatile_ptr: *mut uint8_t = ptr as *mut uint8_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        ::core::ptr::write_volatile(volatile_ptr.offset(i as isize), 0 as uint8_t);
        i = i.wrapping_add(1);
    }
}
pub const MLK_XOF_RATE: ::core::ffi::c_int = SHAKE128_RATE;
pub const OQS_SHA3_SHAKE128_RATE: ::core::ffi::c_int = 168 as ::core::ffi::c_int;
pub const SHAKE128_RATE: ::core::ffi::c_int = OQS_SHA3_SHAKE128_RATE;
unsafe extern "C" fn mlk_rej_uniform_c(
    mut r: *mut int16_t,
    mut target: ::core::ffi::c_uint,
    mut offset: ::core::ffi::c_uint,
    mut buf: *const uint8_t,
    mut buflen: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut ctr: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    let mut val0: int16_t = 0;
    let mut val1: int16_t = 0;
    ctr = offset;
    pos = 0 as ::core::ffi::c_uint;
    while ctr < target && pos.wrapping_add(3 as ::core::ffi::c_uint) <= buflen {
        val0 = ((*buf.offset(pos.wrapping_add(0 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int >> 0 as ::core::ffi::c_int
            | (*buf.offset(pos.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            & 0xfff as ::core::ffi::c_int) as int16_t;
        val1 = ((*buf.offset(pos.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
            | (*buf.offset(pos.wrapping_add(2 as ::core::ffi::c_uint) as isize)
                as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            & 0xfff as ::core::ffi::c_int) as int16_t;
        pos = pos.wrapping_add(3 as ::core::ffi::c_uint);
        if (val0 as ::core::ffi::c_int) < MLKEM_Q {
            let fresh0 = ctr;
            ctr = ctr.wrapping_add(1);
            *r.offset(fresh0 as isize) = val0;
        }
        if ctr < target && (val1 as ::core::ffi::c_int) < MLKEM_Q {
            let fresh1 = ctr;
            ctr = ctr.wrapping_add(1);
            *r.offset(fresh1 as isize) = val1;
        }
    }
    return ctr;
}
unsafe extern "C" fn mlk_rej_uniform(
    mut r: *mut int16_t,
    mut target: ::core::ffi::c_uint,
    mut offset: ::core::ffi::c_uint,
    mut buf: *const uint8_t,
    mut buflen: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    return mlk_rej_uniform_c(r, target, offset, buf, buflen);
}
pub const MLKEM_GEN_MATRIX_NBLOCKS: uint32_t = ((12 as ::core::ffi::c_int * MLKEM_N
    / 8 as ::core::ffi::c_int) as uint32_t)
    .wrapping_mul((1 as ::core::ffi::c_int as uint32_t) << 12 as ::core::ffi::c_int)
    .wrapping_div(MLKEM_Q as uint32_t)
    .wrapping_add(MLK_XOF_RATE as uint32_t)
    .wrapping_div(MLK_XOF_RATE as uint32_t);
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_rej_uniform_x4(
    mut vec0: *mut mlk_poly,
    mut vec1: *mut mlk_poly,
    mut vec2: *mut mlk_poly,
    mut vec3: *mut mlk_poly,
    mut seed: *mut [uint8_t; 64],
) {
    let mut buf: [[uint8_t; 512]; 4] = [[0; 512]; 4];
    let mut ctr: [::core::ffi::c_uint; 4] = [0; 4];
    let mut statex: OQS_SHA3_shake128_x4_inc_ctx = OQS_SHA3_shake128_x4_inc_ctx {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut buflen: ::core::ffi::c_uint = 0;
    OQS_SHA3_shake128_x4_inc_init(&raw mut statex);
    OQS_SHA3_shake128_x4_absorb_once(
        &raw mut statex,
        &raw mut *seed.offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *seed.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *seed.offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *seed.offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
    );
    OQS_SHA3_shake128_x4_inc_squeeze(
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        ((12 as ::core::ffi::c_int * 256 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
            as uint32_t)
            .wrapping_mul(
                (1 as ::core::ffi::c_int as uint32_t) << 12 as ::core::ffi::c_int,
            )
            .wrapping_div(3329 as uint32_t)
            .wrapping_add(168 as uint32_t)
            .wrapping_div(168 as uint32_t)
            .wrapping_mul(OQS_SHA3_SHAKE128_RATE as uint32_t) as size_t,
        &raw mut statex,
    );
    buflen = MLKEM_GEN_MATRIX_NBLOCKS.wrapping_mul(MLK_XOF_RATE as uint32_t)
        as ::core::ffi::c_uint;
    ctr[0 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
        &raw mut (*vec0).0.coeffs as *mut int16_t,
        MLKEM_N as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        buflen,
    );
    ctr[1 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
        &raw mut (*vec1).0.coeffs as *mut int16_t,
        MLKEM_N as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        buflen,
    );
    ctr[2 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
        &raw mut (*vec2).0.coeffs as *mut int16_t,
        MLKEM_N as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        buflen,
    );
    ctr[3 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
        &raw mut (*vec3).0.coeffs as *mut int16_t,
        MLKEM_N as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        &raw mut *(&raw mut buf as *mut [uint8_t; 512])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        buflen,
    );
    buflen = MLK_XOF_RATE as ::core::ffi::c_uint;
    while ctr[0 as ::core::ffi::c_int as usize] < MLKEM_N as ::core::ffi::c_uint
        || ctr[1 as ::core::ffi::c_int as usize] < MLKEM_N as ::core::ffi::c_uint
        || ctr[2 as ::core::ffi::c_int as usize] < MLKEM_N as ::core::ffi::c_uint
        || ctr[3 as ::core::ffi::c_int as usize] < MLKEM_N as ::core::ffi::c_uint
    {
        OQS_SHA3_shake128_x4_inc_squeeze(
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
            (1 as ::core::ffi::c_int * OQS_SHA3_SHAKE128_RATE) as size_t,
            &raw mut statex,
        );
        ctr[0 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
            &raw mut (*vec0).0.coeffs as *mut int16_t,
            MLKEM_N as ::core::ffi::c_uint,
            ctr[0 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
            buflen,
        );
        ctr[1 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
            &raw mut (*vec1).0.coeffs as *mut int16_t,
            MLKEM_N as ::core::ffi::c_uint,
            ctr[1 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
            buflen,
        );
        ctr[2 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
            &raw mut (*vec2).0.coeffs as *mut int16_t,
            MLKEM_N as ::core::ffi::c_uint,
            ctr[2 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
            buflen,
        );
        ctr[3 as ::core::ffi::c_int as usize] = mlk_rej_uniform(
            &raw mut (*vec3).0.coeffs as *mut int16_t,
            MLKEM_N as ::core::ffi::c_uint,
            ctr[3 as ::core::ffi::c_int as usize],
            &raw mut *(&raw mut buf as *mut [uint8_t; 512])
                .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
            buflen,
        );
    }
    OQS_SHA3_shake128_x4_inc_ctx_release(&raw mut statex);
    mlk_zeroize(
        &raw mut buf as *mut [uint8_t; 512] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 512]; 4]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_rej_uniform(
    mut entry: *mut mlk_poly,
    mut seed: *mut uint8_t,
) {
    let mut state: OQS_SHA3_shake128_inc_ctx = OQS_SHA3_shake128_inc_ctx {
        ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut buf: [uint8_t; 504] = [0; 504];
    let mut ctr: ::core::ffi::c_uint = 0;
    let mut buflen: ::core::ffi::c_uint = 0;
    OQS_SHA3_shake128_inc_init(&raw mut state);
    OQS_SHA3_shake128_absorb_once(
        &raw mut state,
        seed as *const uint8_t,
        (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
    );
    OQS_SHA3_shake128_inc_squeeze(
        &raw mut buf as *mut uint8_t,
        ((12 as ::core::ffi::c_int * 256 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
            as uint32_t)
            .wrapping_mul(
                (1 as ::core::ffi::c_int as uint32_t) << 12 as ::core::ffi::c_int,
            )
            .wrapping_div(3329 as uint32_t)
            .wrapping_add(168 as uint32_t)
            .wrapping_div(168 as uint32_t)
            .wrapping_mul(OQS_SHA3_SHAKE128_RATE as uint32_t) as size_t,
        &raw mut state,
    );
    buflen = MLKEM_GEN_MATRIX_NBLOCKS.wrapping_mul(MLK_XOF_RATE as uint32_t)
        as ::core::ffi::c_uint;
    ctr = mlk_rej_uniform(
        &raw mut (*entry).0.coeffs as *mut int16_t,
        MLKEM_N as ::core::ffi::c_uint,
        0 as ::core::ffi::c_uint,
        &raw mut buf as *mut uint8_t,
        buflen,
    );
    buflen = MLK_XOF_RATE as ::core::ffi::c_uint;
    while ctr < MLKEM_N as ::core::ffi::c_uint {
        OQS_SHA3_shake128_inc_squeeze(
            &raw mut buf as *mut uint8_t,
            (1 as ::core::ffi::c_int * OQS_SHA3_SHAKE128_RATE) as size_t,
            &raw mut state,
        );
        ctr = mlk_rej_uniform(
            &raw mut (*entry).0.coeffs as *mut int16_t,
            MLKEM_N as ::core::ffi::c_uint,
            ctr,
            &raw mut buf as *mut uint8_t,
            buflen,
        );
    }
    OQS_SHA3_shake128_inc_ctx_release(&raw mut state);
    mlk_zeroize(
        &raw mut buf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 504]>() as size_t,
    );
}
unsafe extern "C" fn mlk_load32_littleendian(mut x: *const uint8_t) -> uint32_t {
    let mut r: uint32_t = 0;
    r = *x.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    r
        |= (*x.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int;
    r
        |= (*x.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int;
    r
        |= (*x.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_cbd2(
    mut r: *mut mlk_poly,
    mut buf: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut j: ::core::ffi::c_uint = 0;
        let mut t: uint32_t = mlk_load32_littleendian(
            buf.offset((4 as ::core::ffi::c_uint).wrapping_mul(i) as isize),
        );
        let mut d: uint32_t = t & 0x55555555 as uint32_t;
        d = d.wrapping_add(t >> 1 as ::core::ffi::c_int & 0x55555555 as uint32_t);
        j = 0 as ::core::ffi::c_uint;
        while j < 8 as ::core::ffi::c_uint {
            let a: int16_t = (d
                >> (4 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(0 as ::core::ffi::c_uint) & 0x3 as uint32_t)
                as int16_t;
            let b: int16_t = (d
                >> (4 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(2 as ::core::ffi::c_uint) & 0x3 as uint32_t)
                as int16_t;
            (*r)
                .0
                .coeffs[(8 as ::core::ffi::c_uint).wrapping_mul(i).wrapping_add(j)
                as usize] = (a as ::core::ffi::c_int - b as ::core::ffi::c_int)
                as int16_t;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
