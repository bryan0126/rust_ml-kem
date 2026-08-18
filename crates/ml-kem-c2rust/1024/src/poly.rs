extern "C" {
    static mut PQCP_MLKEM_NATIVE_MLKEM1024_C_ct_opt_blocker_u64: uint64_t;
}
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const MLKEM_N: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MLKEM_Q: ::core::ffi::c_int = 3329 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mlk_ct_get_optblocker_u64() -> uint64_t {
    return PQCP_MLKEM_NATIVE_MLKEM1024_C_ct_opt_blocker_u64;
}
#[inline]
unsafe extern "C" fn mlk_ct_get_optblocker_i32() -> int32_t {
    return mlk_ct_get_optblocker_u64() as int32_t;
}
#[inline]
unsafe extern "C" fn mlk_value_barrier_i32(mut b: int32_t) -> int32_t {
    return b ^ mlk_ct_get_optblocker_i32();
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
unsafe extern "C" fn mlk_ct_cmask_neg_i16(mut x: int16_t) -> uint16_t {
    let mut tmp: int32_t = mlk_value_barrier_i32(x as int32_t);
    tmp >>= 16 as ::core::ffi::c_int;
    return mlk_cast_int32_to_uint16(tmp);
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
unsafe extern "C" fn mlk_fqmul(mut a: int16_t, mut b: int16_t) -> int16_t {
    let mut res: int16_t = 0;
    res = mlk_montgomery_reduce(a as int32_t * b as int32_t);
    return res;
}
#[inline]
unsafe extern "C" fn mlk_barrett_reduce(mut a: int16_t) -> int16_t {
    let magic: int32_t = 20159 as int32_t;
    let t: int32_t = magic * a as int32_t
        + ((1 as ::core::ffi::c_int as int32_t) << 25 as ::core::ffi::c_int)
        >> 26 as ::core::ffi::c_int;
    let mut res: int16_t = (a as int32_t - t * MLKEM_Q as int32_t) as int16_t;
    return res;
}
unsafe extern "C" fn mlk_poly_tomont_c(mut r: *mut mlk_poly) {
    let mut i: ::core::ffi::c_uint = 0;
    let f: int16_t = 1353 as int16_t;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_N as ::core::ffi::c_uint {
        (*r).0.coeffs[i as usize] = mlk_fqmul((*r).0.coeffs[i as usize], f);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_tomont(
    mut r: *mut mlk_poly,
) {
    mlk_poly_tomont_c(r);
}
#[inline]
unsafe extern "C" fn mlk_scalar_signed_to_unsigned_q(mut c: int16_t) -> int16_t {
    c = mlk_ct_sel_int16(
        (c as ::core::ffi::c_int + MLKEM_Q) as int16_t,
        c,
        mlk_ct_cmask_neg_i16(c),
    );
    return c;
}
unsafe extern "C" fn mlk_poly_reduce_c(mut r: *mut mlk_poly) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_N as ::core::ffi::c_uint {
        let mut t: int16_t = mlk_barrett_reduce((*r).0.coeffs[i as usize]);
        (*r).0.coeffs[i as usize] = mlk_scalar_signed_to_unsigned_q(t);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_reduce(
    mut r: *mut mlk_poly,
) {
    mlk_poly_reduce_c(r);
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_add(
    mut r: *mut mlk_poly,
    mut b: *const mlk_poly,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_N as ::core::ffi::c_uint {
        (*r).0.coeffs[i as usize] = ((*r).0.coeffs[i as usize] as ::core::ffi::c_int
            + (*b).0.coeffs[i as usize] as ::core::ffi::c_int) as int16_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_sub(
    mut r: *mut mlk_poly,
    mut b: *const mlk_poly,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_N as ::core::ffi::c_uint {
        (*r).0.coeffs[i as usize] = ((*r).0.coeffs[i as usize] as ::core::ffi::c_int
            - (*b).0.coeffs[i as usize] as ::core::ffi::c_int) as int16_t;
        i = i.wrapping_add(1);
    }
}
static mut mlk_zetas: [int16_t; 128] = [
    -(1044 as ::core::ffi::c_int) as int16_t,
    -(758 as ::core::ffi::c_int) as int16_t,
    -(359 as ::core::ffi::c_int) as int16_t,
    -(1517 as ::core::ffi::c_int) as int16_t,
    1493 as ::core::ffi::c_int as int16_t,
    1422 as ::core::ffi::c_int as int16_t,
    287 as ::core::ffi::c_int as int16_t,
    202 as ::core::ffi::c_int as int16_t,
    -(171 as ::core::ffi::c_int) as int16_t,
    622 as ::core::ffi::c_int as int16_t,
    1577 as ::core::ffi::c_int as int16_t,
    182 as ::core::ffi::c_int as int16_t,
    962 as ::core::ffi::c_int as int16_t,
    -(1202 as ::core::ffi::c_int) as int16_t,
    -(1474 as ::core::ffi::c_int) as int16_t,
    1468 as ::core::ffi::c_int as int16_t,
    573 as ::core::ffi::c_int as int16_t,
    -(1325 as ::core::ffi::c_int) as int16_t,
    264 as ::core::ffi::c_int as int16_t,
    383 as ::core::ffi::c_int as int16_t,
    -(829 as ::core::ffi::c_int) as int16_t,
    1458 as ::core::ffi::c_int as int16_t,
    -(1602 as ::core::ffi::c_int) as int16_t,
    -(130 as ::core::ffi::c_int) as int16_t,
    -(681 as ::core::ffi::c_int) as int16_t,
    1017 as ::core::ffi::c_int as int16_t,
    732 as ::core::ffi::c_int as int16_t,
    608 as ::core::ffi::c_int as int16_t,
    -(1542 as ::core::ffi::c_int) as int16_t,
    411 as ::core::ffi::c_int as int16_t,
    -(205 as ::core::ffi::c_int) as int16_t,
    -(1571 as ::core::ffi::c_int) as int16_t,
    1223 as ::core::ffi::c_int as int16_t,
    652 as ::core::ffi::c_int as int16_t,
    -(552 as ::core::ffi::c_int) as int16_t,
    1015 as ::core::ffi::c_int as int16_t,
    -(1293 as ::core::ffi::c_int) as int16_t,
    1491 as ::core::ffi::c_int as int16_t,
    -(282 as ::core::ffi::c_int) as int16_t,
    -(1544 as ::core::ffi::c_int) as int16_t,
    516 as ::core::ffi::c_int as int16_t,
    -(8 as ::core::ffi::c_int) as int16_t,
    -(320 as ::core::ffi::c_int) as int16_t,
    -(666 as ::core::ffi::c_int) as int16_t,
    -(1618 as ::core::ffi::c_int) as int16_t,
    -(1162 as ::core::ffi::c_int) as int16_t,
    126 as ::core::ffi::c_int as int16_t,
    1469 as ::core::ffi::c_int as int16_t,
    -(853 as ::core::ffi::c_int) as int16_t,
    -(90 as ::core::ffi::c_int) as int16_t,
    -(271 as ::core::ffi::c_int) as int16_t,
    830 as ::core::ffi::c_int as int16_t,
    107 as ::core::ffi::c_int as int16_t,
    -(1421 as ::core::ffi::c_int) as int16_t,
    -(247 as ::core::ffi::c_int) as int16_t,
    -(951 as ::core::ffi::c_int) as int16_t,
    -(398 as ::core::ffi::c_int) as int16_t,
    961 as ::core::ffi::c_int as int16_t,
    -(1508 as ::core::ffi::c_int) as int16_t,
    -(725 as ::core::ffi::c_int) as int16_t,
    448 as ::core::ffi::c_int as int16_t,
    -(1065 as ::core::ffi::c_int) as int16_t,
    677 as ::core::ffi::c_int as int16_t,
    -(1275 as ::core::ffi::c_int) as int16_t,
    -(1103 as ::core::ffi::c_int) as int16_t,
    430 as ::core::ffi::c_int as int16_t,
    555 as ::core::ffi::c_int as int16_t,
    843 as ::core::ffi::c_int as int16_t,
    -(1251 as ::core::ffi::c_int) as int16_t,
    871 as ::core::ffi::c_int as int16_t,
    1550 as ::core::ffi::c_int as int16_t,
    105 as ::core::ffi::c_int as int16_t,
    422 as ::core::ffi::c_int as int16_t,
    587 as ::core::ffi::c_int as int16_t,
    177 as ::core::ffi::c_int as int16_t,
    -(235 as ::core::ffi::c_int) as int16_t,
    -(291 as ::core::ffi::c_int) as int16_t,
    -(460 as ::core::ffi::c_int) as int16_t,
    1574 as ::core::ffi::c_int as int16_t,
    1653 as ::core::ffi::c_int as int16_t,
    -(246 as ::core::ffi::c_int) as int16_t,
    778 as ::core::ffi::c_int as int16_t,
    1159 as ::core::ffi::c_int as int16_t,
    -(147 as ::core::ffi::c_int) as int16_t,
    -(777 as ::core::ffi::c_int) as int16_t,
    1483 as ::core::ffi::c_int as int16_t,
    -(602 as ::core::ffi::c_int) as int16_t,
    1119 as ::core::ffi::c_int as int16_t,
    -(1590 as ::core::ffi::c_int) as int16_t,
    644 as ::core::ffi::c_int as int16_t,
    -(872 as ::core::ffi::c_int) as int16_t,
    349 as ::core::ffi::c_int as int16_t,
    418 as ::core::ffi::c_int as int16_t,
    329 as ::core::ffi::c_int as int16_t,
    -(156 as ::core::ffi::c_int) as int16_t,
    -(75 as ::core::ffi::c_int) as int16_t,
    817 as ::core::ffi::c_int as int16_t,
    1097 as ::core::ffi::c_int as int16_t,
    603 as ::core::ffi::c_int as int16_t,
    610 as ::core::ffi::c_int as int16_t,
    1322 as ::core::ffi::c_int as int16_t,
    -(1285 as ::core::ffi::c_int) as int16_t,
    -(1465 as ::core::ffi::c_int) as int16_t,
    384 as ::core::ffi::c_int as int16_t,
    -(1215 as ::core::ffi::c_int) as int16_t,
    -(136 as ::core::ffi::c_int) as int16_t,
    1218 as ::core::ffi::c_int as int16_t,
    -(1335 as ::core::ffi::c_int) as int16_t,
    -(874 as ::core::ffi::c_int) as int16_t,
    220 as ::core::ffi::c_int as int16_t,
    -(1187 as ::core::ffi::c_int) as int16_t,
    -(1659 as ::core::ffi::c_int) as int16_t,
    -(1185 as ::core::ffi::c_int) as int16_t,
    -(1530 as ::core::ffi::c_int) as int16_t,
    -(1278 as ::core::ffi::c_int) as int16_t,
    794 as ::core::ffi::c_int as int16_t,
    -(1510 as ::core::ffi::c_int) as int16_t,
    -(854 as ::core::ffi::c_int) as int16_t,
    -(870 as ::core::ffi::c_int) as int16_t,
    478 as ::core::ffi::c_int as int16_t,
    -(108 as ::core::ffi::c_int) as int16_t,
    -(308 as ::core::ffi::c_int) as int16_t,
    996 as ::core::ffi::c_int as int16_t,
    991 as ::core::ffi::c_int as int16_t,
    958 as ::core::ffi::c_int as int16_t,
    -(1460 as ::core::ffi::c_int) as int16_t,
    1522 as ::core::ffi::c_int as int16_t,
    1628 as ::core::ffi::c_int as int16_t,
];
unsafe extern "C" fn mlk_poly_mulcache_compute_c(
    mut x: *mut mlk_poly_mulcache,
    mut a: *const mlk_poly,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (MLKEM_N / 4 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        (*x)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(0 as ::core::ffi::c_uint) as usize] = mlk_fqmul(
            (*a)
                .0
                .coeffs[(4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(1 as ::core::ffi::c_uint) as usize],
            mlk_zetas[(64 as ::core::ffi::c_uint).wrapping_add(i) as usize],
        );
        (*x)
            .0
            .coeffs[(2 as ::core::ffi::c_uint)
            .wrapping_mul(i)
            .wrapping_add(1 as ::core::ffi::c_uint) as usize] = mlk_fqmul(
            (*a)
                .0
                .coeffs[(4 as ::core::ffi::c_uint)
                .wrapping_mul(i)
                .wrapping_add(3 as ::core::ffi::c_uint) as usize],
            -(mlk_zetas[(64 as ::core::ffi::c_uint).wrapping_add(i) as usize]
                as ::core::ffi::c_int) as int16_t,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_mulcache_compute(
    mut x: *mut mlk_poly_mulcache,
    mut a: *const mlk_poly,
) {
    mlk_poly_mulcache_compute_c(x, a);
}
unsafe extern "C" fn mlk_ntt_butterfly_block(
    mut r: *mut int16_t,
    mut zeta: int16_t,
    mut start: ::core::ffi::c_uint,
    mut len: ::core::ffi::c_uint,
    mut bound: ::core::ffi::c_uint,
) {
    let mut j: ::core::ffi::c_uint = 0;
    j = start;
    while j < start.wrapping_add(len) {
        let mut t: int16_t = 0;
        t = mlk_fqmul(*r.offset(j.wrapping_add(len) as isize), zeta);
        *r.offset(j.wrapping_add(len) as isize) = (*r.offset(j as isize)
            as ::core::ffi::c_int - t as ::core::ffi::c_int) as int16_t;
        *r.offset(j as isize) = (*r.offset(j as isize) as ::core::ffi::c_int
            + t as ::core::ffi::c_int) as int16_t;
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn mlk_ntt_layer(mut r: *mut int16_t, mut layer: ::core::ffi::c_uint) {
    let mut start: ::core::ffi::c_uint = 0;
    let mut k: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    k = (1 as ::core::ffi::c_uint) << layer.wrapping_sub(1 as ::core::ffi::c_uint);
    len = MLKEM_N as ::core::ffi::c_uint >> layer;
    start = 0 as ::core::ffi::c_uint;
    while start < MLKEM_N as ::core::ffi::c_uint {
        let fresh0 = k;
        k = k.wrapping_add(1);
        let mut zeta: int16_t = mlk_zetas[fresh0 as usize];
        mlk_ntt_butterfly_block(
            r,
            zeta,
            start,
            len,
            layer.wrapping_mul(MLKEM_Q as ::core::ffi::c_uint),
        );
        start = start.wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(len));
    }
}
unsafe extern "C" fn mlk_poly_ntt_c(mut p: *mut mlk_poly) {
    let mut layer: ::core::ffi::c_uint = 0;
    let mut r: *mut int16_t = ::core::ptr::null_mut::<int16_t>();
    r = &raw mut (*p).0.coeffs as *mut int16_t;
    layer = 1 as ::core::ffi::c_uint;
    while layer <= 7 as ::core::ffi::c_uint {
        mlk_ntt_layer(r as *mut int16_t, layer);
        layer = layer.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_ntt(mut r: *mut mlk_poly) {
    mlk_poly_ntt_c(r);
}
unsafe extern "C" fn mlk_invntt_layer(
    mut r: *mut int16_t,
    mut layer: ::core::ffi::c_uint,
) {
    let mut start: ::core::ffi::c_uint = 0;
    let mut k: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    len = MLKEM_N as ::core::ffi::c_uint >> layer;
    k = ((1 as ::core::ffi::c_uint) << layer).wrapping_sub(1 as ::core::ffi::c_uint);
    start = 0 as ::core::ffi::c_uint;
    while start < MLKEM_N as ::core::ffi::c_uint {
        let mut j: ::core::ffi::c_uint = 0;
        let fresh1 = k;
        k = k.wrapping_sub(1);
        let mut zeta: int16_t = mlk_zetas[fresh1 as usize];
        j = start;
        while j < start.wrapping_add(len) {
            let mut t: int16_t = *r.offset(j as isize);
            *r.offset(j as isize) = mlk_barrett_reduce(
                (t as ::core::ffi::c_int
                    + *r.offset(j.wrapping_add(len) as isize) as ::core::ffi::c_int)
                    as int16_t,
            );
            *r.offset(j.wrapping_add(len) as isize) = (*r
                .offset(j.wrapping_add(len) as isize) as ::core::ffi::c_int
                - t as ::core::ffi::c_int) as int16_t;
            *r.offset(j.wrapping_add(len) as isize) = mlk_fqmul(
                *r.offset(j.wrapping_add(len) as isize),
                zeta,
            );
            j = j.wrapping_add(1);
        }
        start = start.wrapping_add((2 as ::core::ffi::c_uint).wrapping_mul(len));
    }
}
unsafe extern "C" fn mlk_poly_invntt_tomont_c(mut p: *mut mlk_poly) {
    let mut j: ::core::ffi::c_uint = 0;
    let mut layer: ::core::ffi::c_uint = 0;
    let f: int16_t = 1441 as int16_t;
    let mut r: *mut int16_t = &raw mut (*p).0.coeffs as *mut int16_t;
    j = 0 as ::core::ffi::c_uint;
    while j < MLKEM_N as ::core::ffi::c_uint {
        *r.offset(j as isize) = mlk_fqmul(*r.offset(j as isize), f);
        j = j.wrapping_add(1);
    }
    layer = 7 as ::core::ffi::c_uint;
    while layer > 0 as ::core::ffi::c_uint {
        mlk_invntt_layer(r, layer);
        layer = layer.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM1024_C_poly_invntt_tomont(
    mut r: *mut mlk_poly,
) {
    mlk_poly_invntt_tomont_c(r);
}
