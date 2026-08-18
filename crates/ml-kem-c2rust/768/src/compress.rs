extern "C" {
    static mut PQCP_MLKEM_NATIVE_MLKEM768_C_ct_opt_blocker_u64: uint64_t;
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const MLKEM_N: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MLKEM_Q: ::core::ffi::c_int = 3329 as ::core::ffi::c_int;
pub const MLKEM_Q_HALF: ::core::ffi::c_int = (MLKEM_Q + 1 as ::core::ffi::c_int)
    / 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mlk_ct_get_optblocker_u64() -> uint64_t {
    return PQCP_MLKEM_NATIVE_MLKEM768_C_ct_opt_blocker_u64;
}
#[inline]
unsafe extern "C" fn mlk_ct_get_optblocker_u8() -> uint8_t {
    return mlk_ct_get_optblocker_u64() as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_ct_get_optblocker_i32() -> int32_t {
    return mlk_ct_get_optblocker_u64() as int32_t;
}
#[inline]
unsafe extern "C" fn mlk_value_barrier_i32(mut b: int32_t) -> int32_t {
    return b ^ mlk_ct_get_optblocker_i32();
}
#[inline]
unsafe extern "C" fn mlk_value_barrier_u8(mut b: uint8_t) -> uint8_t {
    return (b as ::core::ffi::c_int ^ mlk_ct_get_optblocker_u8() as ::core::ffi::c_int)
        as uint8_t;
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
unsafe extern "C" fn mlk_cast_int16_to_uint16(mut x: int32_t) -> uint16_t {
    return mlk_cast_int32_to_uint16(x);
}
#[inline]
unsafe extern "C" fn mlk_ct_cmask_nonzero_u16(mut x: uint16_t) -> uint16_t {
    let mut tmp: int32_t = mlk_value_barrier_i32(-(x as int32_t));
    tmp >>= 16 as ::core::ffi::c_int;
    return mlk_cast_int32_to_uint16(tmp);
}
#[inline]
unsafe extern "C" fn mlk_ct_sel_int16(
    mut a: int16_t,
    mut b: int16_t,
    mut cond: uint16_t,
) -> int16_t {
    let mut au: uint16_t = mlk_cast_int16_to_uint16(a as int32_t);
    let mut bu: uint16_t = mlk_cast_int16_to_uint16(b as int32_t);
    let mut res: uint16_t = (bu as ::core::ffi::c_int
        ^ mlk_ct_cmask_nonzero_u16(cond) as ::core::ffi::c_int
            & (au as ::core::ffi::c_int ^ bu as ::core::ffi::c_int)) as uint16_t;
    return mlk_cast_uint16_to_int16(res);
}
#[inline]
unsafe extern "C" fn mlk_scalar_compress_d1(mut u: int16_t) -> uint8_t {
    let mut d0: uint32_t = (u as uint32_t).wrapping_mul(1290168 as uint32_t);
    return (d0
        .wrapping_add((1 as ::core::ffi::c_uint as uint32_t) << 30 as ::core::ffi::c_int)
        >> 31 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_scalar_compress_d4(mut u: int16_t) -> uint8_t {
    let mut d0: uint32_t = (u as uint32_t).wrapping_mul(1290160 as uint32_t);
    return (d0
        .wrapping_add((1 as ::core::ffi::c_uint as uint32_t) << 27 as ::core::ffi::c_int)
        >> 28 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_scalar_decompress_d4(mut u: uint8_t) -> int16_t {
    return ((u as uint32_t).wrapping_mul(MLKEM_Q as uint32_t).wrapping_add(8 as uint32_t)
        >> 4 as ::core::ffi::c_int) as int16_t;
}
#[inline]
unsafe extern "C" fn mlk_scalar_compress_d10(mut u: int16_t) -> uint16_t {
    let mut d0: uint64_t = (u as uint64_t).wrapping_mul(2642263040 as uint64_t);
    d0 = d0
        .wrapping_add((1 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int)
        >> 33 as ::core::ffi::c_int;
    return (d0 & 0x3ff as uint64_t) as uint16_t;
}
#[inline]
unsafe extern "C" fn mlk_scalar_decompress_d10(mut u: uint16_t) -> int16_t {
    return ((u as uint32_t)
        .wrapping_mul(MLKEM_Q as uint32_t)
        .wrapping_add(512 as uint32_t) >> 10 as ::core::ffi::c_int) as int16_t;
}
unsafe extern "C" fn mlk_poly_compress_d4_c(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut j: ::core::ffi::c_uint = 0;
        let mut t: [uint8_t; 8] = [
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        j = 0 as ::core::ffi::c_uint;
        while j < 8 as ::core::ffi::c_uint {
            t[j as usize] = mlk_scalar_compress_d4(
                (*a)
                    .0
                    .coeffs[(8 as ::core::ffi::c_uint).wrapping_mul(i).wrapping_add(j)
                    as usize],
            );
            j = j.wrapping_add(1);
        }
        *r.offset(i.wrapping_mul(4 as ::core::ffi::c_uint) as isize) = (t[0
            as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            | (t[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                i
                    .wrapping_mul(4 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as isize,
            ) = (t[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            | (t[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                i
                    .wrapping_mul(4 as ::core::ffi::c_uint)
                    .wrapping_add(2 as ::core::ffi::c_uint) as isize,
            ) = (t[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            | (t[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                i
                    .wrapping_mul(4 as ::core::ffi::c_uint)
                    .wrapping_add(3 as ::core::ffi::c_uint) as isize,
            ) = (t[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            | (t[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_d4(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    mlk_poly_compress_d4_c(r, a);
}
unsafe extern "C" fn mlk_poly_compress_d10_c(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    let mut j: ::core::ffi::c_uint = 0;
    j = 0 as ::core::ffi::c_uint;
    while j < (MLKEM_N / 4 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut k: ::core::ffi::c_uint = 0;
        let mut t: [uint16_t; 4] = [0; 4];
        k = 0 as ::core::ffi::c_uint;
        while k < 4 as ::core::ffi::c_uint {
            t[k as usize] = mlk_scalar_compress_d10(
                (*a)
                    .0
                    .coeffs[(4 as ::core::ffi::c_uint).wrapping_mul(j).wrapping_add(k)
                    as usize],
            );
            k = k.wrapping_add(1);
        }
        *r
            .offset(
                (5 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(0 as ::core::ffi::c_uint) as isize,
            ) = (t[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (5 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(1 as ::core::ffi::c_uint) as isize,
            ) = (t[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int
            | (t[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (5 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(2 as ::core::ffi::c_uint) as isize,
            ) = (t[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int
            | (t[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (5 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(3 as ::core::ffi::c_uint) as isize,
            ) = (t[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int
            | (t[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (5 as ::core::ffi::c_uint)
                    .wrapping_mul(j)
                    .wrapping_add(4 as ::core::ffi::c_uint) as isize,
            ) = (t[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint8_t;
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_d10(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    mlk_poly_compress_d10_c(r, a);
}
unsafe extern "C" fn mlk_poly_decompress_d4_c(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(0 as ::core::ffi::c_uint) as usize] = mlk_scalar_decompress_d4(
            (*a.offset(i as isize) as ::core::ffi::c_int >> 0 as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t,
        );
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize] = mlk_scalar_decompress_d4(
            (*a.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_d4(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    mlk_poly_decompress_d4_c(r, a);
}
unsafe extern "C" fn mlk_poly_decompress_d10_c(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    let mut j: ::core::ffi::c_uint = 0;
    j = 0 as ::core::ffi::c_uint;
    while j < (MLKEM_N / 4 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut k: ::core::ffi::c_uint = 0;
        let mut t: [uint16_t; 4] = [0; 4];
        let mut base: *const uint8_t = a
            .offset((5 as ::core::ffi::c_uint).wrapping_mul(j) as isize)
            as *const uint8_t;
        t[0 as ::core::ffi::c_int as usize] = (0x3ff as ::core::ffi::c_int
            & (*base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 0 as ::core::ffi::c_int
                | (*base.offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)) as uint16_t;
        t[1 as ::core::ffi::c_int as usize] = (0x3ff as ::core::ffi::c_int
            & (*base.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int
                | (*base.offset(2 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)) as uint16_t;
        t[2 as ::core::ffi::c_int as usize] = (0x3ff as ::core::ffi::c_int
            & (*base.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int
                | (*base.offset(3 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)) as uint16_t;
        t[3 as ::core::ffi::c_int as usize] = (0x3ff as ::core::ffi::c_int
            & (*base.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int
                | (*base.offset(4 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)) as uint16_t;
        k = 0 as ::core::ffi::c_uint;
        while k < 4 as ::core::ffi::c_uint {
            (*r)
                .0
                .coeffs[(4 as ::core::ffi::c_uint).wrapping_mul(j).wrapping_add(k)
                as usize] = mlk_scalar_decompress_d10(t[k as usize]);
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_d10(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    mlk_poly_decompress_d10_c(r, a);
}
unsafe extern "C" fn mlk_poly_tobytes_c(mut r: *mut uint8_t, mut a: *const mlk_poly) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let t0: uint16_t = (*a)
            .0
            .coeffs[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize] as uint16_t;
        let t1: uint16_t = (*a)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize] as uint16_t;
        *r
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(0 as ::core::ffi::c_uint) as isize,
            ) = (t0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as ::core::ffi::c_uint) as isize,
            ) = (t0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (t1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int) as uint8_t;
        *r
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(2 as ::core::ffi::c_uint) as isize,
            ) = (t1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_tobytes(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    mlk_poly_tobytes_c(r, a);
}
unsafe extern "C" fn mlk_poly_frombytes_c(mut r: *mut mlk_poly, mut a: *const uint8_t) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let t0: uint8_t = *a
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(0 as ::core::ffi::c_uint) as isize,
            );
        let t1: uint8_t = *a
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as ::core::ffi::c_uint) as isize,
            );
        let t2: uint8_t = *a
            .offset(
                (3 as ::core::ffi::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(2 as ::core::ffi::c_uint) as isize,
            );
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(0 as ::core::ffi::c_uint) as usize] = (t0 as ::core::ffi::c_int
            | (t1 as uint16_t as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                & 0xfff as ::core::ffi::c_int) as int16_t;
        (*r)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize] = (t1 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int
            | (t2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as int16_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_frombytes(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    mlk_poly_frombytes_c(r, a);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_frommsg(
    mut r: *mut mlk_poly,
    mut msg: *const uint8_t,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut j: ::core::ffi::c_uint = 0;
        j = 0 as ::core::ffi::c_uint;
        while j < 8 as ::core::ffi::c_uint {
            let mut mask: uint8_t = mlk_value_barrier_u8(
                ((1 as ::core::ffi::c_uint) << j) as uint8_t,
            );
            (*r)
                .0
                .coeffs[(8 as ::core::ffi::c_uint).wrapping_mul(i).wrapping_add(j)
                as usize] = mlk_ct_sel_int16(
                MLKEM_Q_HALF as int16_t,
                0 as int16_t,
                (*msg.offset(i as isize) as ::core::ffi::c_int
                    & mask as ::core::ffi::c_int) as uint16_t,
            );
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_tomsg(
    mut msg: *mut uint8_t,
    mut r: *const mlk_poly,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        let mut j: ::core::ffi::c_uint = 0;
        *msg.offset(i as isize) = 0 as uint8_t;
        j = 0 as ::core::ffi::c_uint;
        while j < 8 as ::core::ffi::c_uint {
            let mut t: uint32_t = mlk_scalar_compress_d1(
                (*r)
                    .0
                    .coeffs[(8 as ::core::ffi::c_uint).wrapping_mul(i).wrapping_add(j)
                    as usize],
            ) as uint32_t;
            let ref mut fresh0 = *msg.offset(i as isize);
            *fresh0 = (*fresh0 as ::core::ffi::c_int
                | (t << j) as uint8_t as ::core::ffi::c_int) as uint8_t;
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
