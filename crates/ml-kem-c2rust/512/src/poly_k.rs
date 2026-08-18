extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_tomont(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_mulcache_compute(
        x: *mut mlk_poly_mulcache,
        a: *const mlk_poly,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_reduce(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_add(r: *mut mlk_poly, b: *const mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_ntt(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_invntt_tomont(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_compress_d10(
        r: *mut uint8_t,
        a: *const mlk_poly,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_decompress_d10(
        r: *mut mlk_poly,
        a: *const uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_tobytes(r: *mut uint8_t, a: *const mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_frombytes(r: *mut mlk_poly, a: *const uint8_t);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_cbd2(r: *mut mlk_poly, buf: *const uint8_t);
    fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_cbd3(r: *mut mlk_poly, buf: *const uint8_t);
    fn OQS_SHA3_shake256(
        output: *mut uint8_t,
        outlen: size_t,
        input: *const uint8_t,
        inplen: size_t,
    );
    fn OQS_SHA3_shake256_x4(
        out0: *mut uint8_t,
        out1: *mut uint8_t,
        out2: *mut uint8_t,
        out3: *mut uint8_t,
        outlen: size_t,
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
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
#[repr(C, align(32))]
pub struct mlk_poly_mulcache(pub mlk_poly_mulcache_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_poly_mulcache_Inner {
    pub coeffs: [int16_t; 128],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_poly_mulcache_PADDING: usize = ::core::mem::size_of::<mlk_poly_mulcache>()
    - ::core::mem::size_of::<mlk_poly_mulcache_Inner>();
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_polyvec(pub mlk_polyvec_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_polyvec_Inner {
    pub vec: [mlk_poly; 2],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polyvec_PADDING: usize = ::core::mem::size_of::<mlk_polyvec>()
    - ::core::mem::size_of::<mlk_polyvec_Inner>();
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_polyvec_mulcache(pub mlk_polyvec_mulcache_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_polyvec_mulcache_Inner {
    pub vec: [mlk_poly_mulcache; 2],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polyvec_mulcache_PADDING: usize = ::core::mem::size_of::<
    mlk_polyvec_mulcache,
>() - ::core::mem::size_of::<mlk_polyvec_mulcache_Inner>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const MLKEM_K: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MLKEM_N: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MLKEM_Q: ::core::ffi::c_int = 3329 as ::core::ffi::c_int;
pub const MLKEM_SYMBYTES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MLKEM_POLYBYTES: ::core::ffi::c_int = 384 as ::core::ffi::c_int;
pub const MLKEM_POLYCOMPRESSEDBYTES_D10: ::core::ffi::c_int = 320 as ::core::ffi::c_int;
pub const MLKEM_ETA1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MLKEM_POLYCOMPRESSEDBYTES_DU: ::core::ffi::c_int = MLKEM_POLYCOMPRESSEDBYTES_D10;
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
#[inline(always)]
unsafe extern "C" fn mlk_cast_uint16_to_int16(mut x: uint16_t) -> int16_t {
    return x as int16_t;
}
#[inline(always)]
unsafe extern "C" fn mlk_cast_int32_to_uint16(mut x: int32_t) -> uint16_t {
    return (x & UINT16_MAX as int32_t) as uint16_t;
}
#[inline(always)]
unsafe extern "C" fn mlk_montgomery_reduce(mut a: int32_t) -> int16_t {
    let QINV: uint32_t = 62209 as uint32_t;
    let a_reduced: uint16_t = mlk_cast_int32_to_uint16(a) as uint16_t;
    let a_inverted: uint16_t = ((a_reduced as uint32_t).wrapping_mul(QINV)
        & UINT16_MAX as uint32_t) as uint16_t;
    let t: int16_t = mlk_cast_uint16_to_int16(a_inverted) as int16_t;
    let mut r: int32_t = 0;
    r = a - t as int32_t * MLKEM_Q as int32_t;
    r = r >> 16 as ::core::ffi::c_int;
    return r as int16_t;
}
#[inline]
unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_compress_du(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    PQCP_MLKEM_NATIVE_MLKEM512_C_poly_compress_d10(r, a);
}
#[inline]
unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_decompress_du(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM512_C_poly_decompress_d10(r, a);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_compress_du(
    mut r: *mut uint8_t,
    mut a: *const mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_compress_du(
            r
                .offset(
                    i.wrapping_mul(MLKEM_POLYCOMPRESSEDBYTES_DU as ::core::ffi::c_uint)
                        as isize,
                ),
            (&raw const (*a).0.vec as *const mlk_poly).offset(i as isize)
                as *const mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_decompress_du(
    mut r: *mut mlk_polyvec,
    mut a: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_decompress_du(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
            a
                .offset(
                    i.wrapping_mul(MLKEM_POLYCOMPRESSEDBYTES_DU as ::core::ffi::c_uint)
                        as isize,
                ),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_tobytes(
    mut r: *mut uint8_t,
    mut a: *const mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_tobytes(
            r.offset(i.wrapping_mul(MLKEM_POLYBYTES as ::core::ffi::c_uint) as isize)
                as *mut uint8_t,
            (&raw const (*a).0.vec as *const mlk_poly).offset(i as isize)
                as *const mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_frombytes(
    mut r: *mut mlk_polyvec,
    mut a: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_frombytes(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
            a.offset(i.wrapping_mul(MLKEM_POLYBYTES as ::core::ffi::c_uint) as isize),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_ntt(
    mut r: *mut mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_ntt(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_invntt_tomont(
    mut r: *mut mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_invntt_tomont(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mlk_polyvec_basemul_acc_montgomery_cached_c(
    mut r: *mut mlk_poly,
    mut a: *const mlk_polyvec,
    mut b: *const mlk_polyvec,
    mut b_cache: *const mlk_polyvec_mulcache,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut k: ::core::ffi::c_uint = 0;
        let mut t: [int32_t; 2] = [0 as ::core::ffi::c_int; 2];
        k = 0 as ::core::ffi::c_uint;
        while k < MLKEM_K as ::core::ffi::c_uint {
            t[0 as ::core::ffi::c_int as usize] = (t[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + ((*a)
                    .0
                    .vec[k as usize]
                    .0
                    .coeffs[(2 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize] as int32_t
                    * (*b_cache).0.vec[k as usize].0.coeffs[i as usize] as int32_t)
                    as ::core::ffi::c_int) as int32_t;
            t[0 as ::core::ffi::c_int as usize] = (t[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + ((*a)
                    .0
                    .vec[k as usize]
                    .0
                    .coeffs[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize]
                    as int32_t
                    * (*b)
                        .0
                        .vec[k as usize]
                        .0
                        .coeffs[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize]
                        as int32_t) as ::core::ffi::c_int) as int32_t;
            t[1 as ::core::ffi::c_int as usize] = (t[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + ((*a)
                    .0
                    .vec[k as usize]
                    .0
                    .coeffs[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize]
                    as int32_t
                    * (*b)
                        .0
                        .vec[k as usize]
                        .0
                        .coeffs[(2 as ::core::ffi::c_uint)
                        .wrapping_mul(i)
                        .wrapping_add(1 as ::core::ffi::c_uint) as usize] as int32_t)
                    as ::core::ffi::c_int) as int32_t;
            t[1 as ::core::ffi::c_int as usize] = (t[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + ((*a)
                    .0
                    .vec[k as usize]
                    .0
                    .coeffs[(2 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize] as int32_t
                    * (*b)
                        .0
                        .vec[k as usize]
                        .0
                        .coeffs[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize]
                        as int32_t) as ::core::ffi::c_int) as int32_t;
            k = k.wrapping_add(1);
        }
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(0 as ::core::ffi::c_uint) as usize] = mlk_montgomery_reduce(
            t[0 as ::core::ffi::c_int as usize],
        );
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize] = mlk_montgomery_reduce(
            t[1 as ::core::ffi::c_int as usize],
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_basemul_acc_montgomery_cached(
    mut r: *mut mlk_poly,
    mut a: *const mlk_polyvec,
    mut b: *const mlk_polyvec,
    mut b_cache: *const mlk_polyvec_mulcache,
) {
    mlk_polyvec_basemul_acc_montgomery_cached_c(r, a, b, b_cache);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_mulcache_compute(
    mut x: *mut mlk_polyvec_mulcache,
    mut a: *const mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_mulcache_compute(
            (&raw mut (*x).0.vec as *mut mlk_poly_mulcache).offset(i as isize)
                as *mut mlk_poly_mulcache,
            (&raw const (*a).0.vec as *const mlk_poly).offset(i as isize)
                as *const mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_reduce(
    mut r: *mut mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_reduce(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_add(
    mut r: *mut mlk_polyvec,
    mut b: *const mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_add(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
            (&raw const (*b).0.vec as *const mlk_poly).offset(i as isize)
                as *const mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_polyvec_tomont(
    mut r: *mut mlk_polyvec,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM512_C_poly_tomont(
            (&raw mut (*r).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
        );
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn mlk_poly_cbd_eta1(mut r: *mut mlk_poly, mut buf: *const uint8_t) {
    PQCP_MLKEM_NATIVE_MLKEM512_C_poly_cbd3(r, buf);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_getnoise_eta1_4x(
    mut r0: *mut mlk_poly,
    mut r1: *mut mlk_poly,
    mut r2: *mut mlk_poly,
    mut r3: *mut mlk_poly,
    mut seed: *const uint8_t,
    mut nonce0: uint8_t,
    mut nonce1: uint8_t,
    mut nonce2: uint8_t,
    mut nonce3: uint8_t,
) {
    let mut buf: [[uint8_t; 192]; 4] = [[0; 192]; 4];
    let mut extkey: [[uint8_t; 64]; 4] = [[0; 64]; 4];
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    extkey[0 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce0;
    extkey[1 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce1;
    extkey[2 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce2;
    extkey[3 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce3;
    OQS_SHA3_shake256_x4(
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        (MLKEM_ETA1 * MLKEM_N / 4 as ::core::ffi::c_int) as size_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        (MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as size_t,
    );
    mlk_poly_cbd_eta1(
        r0,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_poly_cbd_eta1(
        r1,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_poly_cbd_eta1(
        r2,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    if !r3.is_null() {
        mlk_poly_cbd_eta1(
            r3,
            &raw mut *(&raw mut buf as *mut [uint8_t; 192])
                .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t
                as *const uint8_t,
        );
    }
    mlk_zeroize(
        &raw mut buf as *mut [uint8_t; 192] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 192]; 4]>() as size_t,
    );
    mlk_zeroize(
        &raw mut extkey as *mut [uint8_t; 64] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 64]; 4]>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn mlk_poly_cbd_eta2(mut r: *mut mlk_poly, mut buf: *const uint8_t) {
    PQCP_MLKEM_NATIVE_MLKEM512_C_poly_cbd2(r, buf);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_getnoise_eta2(
    mut r: *mut mlk_poly,
    mut seed: *const uint8_t,
    mut nonce: uint8_t,
) {
    let mut buf: [uint8_t; 128] = [0; 128];
    let mut extkey: [uint8_t; 33] = [0; 33];
    memcpy(
        &raw mut extkey as *mut uint8_t as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    extkey[MLKEM_SYMBYTES as usize] = nonce;
    OQS_SHA3_shake256(
        &raw mut buf as *mut uint8_t,
        (2 as ::core::ffi::c_int * MLKEM_N / 4 as ::core::ffi::c_int) as size_t,
        &raw mut extkey as *mut uint8_t,
        (MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as size_t,
    );
    mlk_poly_cbd_eta2(r, &raw mut buf as *mut uint8_t as *const uint8_t);
    mlk_zeroize(
        &raw mut buf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    mlk_zeroize(
        &raw mut extkey as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 33]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM512_C_poly_getnoise_eta1122_4x(
    mut r0: *mut mlk_poly,
    mut r1: *mut mlk_poly,
    mut r2: *mut mlk_poly,
    mut r3: *mut mlk_poly,
    mut seed: *const uint8_t,
    mut nonce0: uint8_t,
    mut nonce1: uint8_t,
    mut nonce2: uint8_t,
    mut nonce3: uint8_t,
) {
    let mut buf: [[uint8_t; 192]; 4] = [[0; 192]; 4];
    let mut extkey: [[uint8_t; 64]; 4] = [[0; 64]; 4];
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    memcpy(
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
    extkey[0 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce0;
    extkey[1 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce1;
    extkey[2 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce2;
    extkey[3 as ::core::ffi::c_int as usize][MLKEM_SYMBYTES as usize] = nonce3;
    OQS_SHA3_shake256_x4(
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        (MLKEM_ETA1 * MLKEM_N / 4 as ::core::ffi::c_int) as size_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
        &raw mut *(&raw mut extkey as *mut [uint8_t; 64])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t,
        (MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as size_t,
    );
    mlk_poly_cbd_eta1(
        r0,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_poly_cbd_eta1(
        r1,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_poly_cbd_eta2(
        r2,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_poly_cbd_eta2(
        r3,
        &raw mut *(&raw mut buf as *mut [uint8_t; 192])
            .offset(3 as ::core::ffi::c_int as isize) as *mut uint8_t as *const uint8_t,
    );
    mlk_zeroize(
        &raw mut buf as *mut [uint8_t; 192] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 192]; 4]>() as size_t,
    );
    mlk_zeroize(
        &raw mut extkey as *mut [uint8_t; 64] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 64]; 4]>() as size_t,
    );
}
