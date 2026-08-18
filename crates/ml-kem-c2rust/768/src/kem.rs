extern "C" {
    fn OQS_randombytes(random_array: *mut uint8_t, bytes_to_read: size_t);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut PQCP_MLKEM_NATIVE_MLKEM768_C_ct_opt_blocker_u64: uint64_t;
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tobytes(
        r: *mut uint8_t,
        a: *const mlk_polyvec,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_frombytes(
        r: *mut mlk_polyvec,
        a: *const uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(r: *mut mlk_polyvec);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_keypair_derand(
        pk: *mut uint8_t,
        sk: *mut uint8_t,
        coins: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_enc(
        c: *mut uint8_t,
        m: *const uint8_t,
        pk: *const uint8_t,
        coins: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_dec(
        m: *mut uint8_t,
        c: *const uint8_t,
        sk: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn OQS_SHA3_sha3_256(output: *mut uint8_t, input: *const uint8_t, inplen: size_t);
    fn OQS_SHA3_sha3_512(output: *mut uint8_t, input: *const uint8_t, inplen: size_t);
    fn OQS_SHA3_shake256(
        output: *mut uint8_t,
        outlen: size_t,
        input: *const uint8_t,
        inplen: size_t,
    );
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_polyvec(pub mlk_polyvec_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_polyvec_Inner {
    pub vec: [mlk_poly; 3],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polyvec_PADDING: usize = ::core::mem::size_of::<mlk_polyvec>()
    - ::core::mem::size_of::<mlk_polyvec_Inner>();
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const MLKEM_K: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MLKEM_SYMBYTES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MLKEM_POLYBYTES: ::core::ffi::c_int = 384 as ::core::ffi::c_int;
pub const MLKEM_POLYVECBYTES: ::core::ffi::c_int = MLKEM_K * MLKEM_POLYBYTES;
pub const MLKEM_POLYCOMPRESSEDBYTES_D4: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MLKEM_POLYCOMPRESSEDBYTES_D10: ::core::ffi::c_int = 320 as ::core::ffi::c_int;
pub const MLKEM_POLYCOMPRESSEDBYTES_DV: ::core::ffi::c_int = MLKEM_POLYCOMPRESSEDBYTES_D4;
pub const MLKEM_POLYCOMPRESSEDBYTES_DU: ::core::ffi::c_int = MLKEM_POLYCOMPRESSEDBYTES_D10;
pub const MLKEM_POLYVECCOMPRESSEDBYTES_DU: ::core::ffi::c_int = MLKEM_K
    * MLKEM_POLYCOMPRESSEDBYTES_DU;
pub const MLKEM_INDCPA_PUBLICKEYBYTES: ::core::ffi::c_int = MLKEM_POLYVECBYTES
    + MLKEM_SYMBYTES;
pub const MLKEM_INDCPA_SECRETKEYBYTES: ::core::ffi::c_int = MLKEM_K * MLKEM_POLYBYTES;
pub const MLKEM_INDCPA_BYTES: ::core::ffi::c_int = MLKEM_POLYVECCOMPRESSEDBYTES_DU
    + MLKEM_POLYCOMPRESSEDBYTES_DV;
pub const MLKEM_INDCCA_PUBLICKEYBYTES: ::core::ffi::c_int = MLKEM_POLYVECBYTES
    + MLKEM_SYMBYTES;
pub const MLKEM_INDCCA_SECRETKEYBYTES: ::core::ffi::c_int = MLKEM_INDCPA_SECRETKEYBYTES
    + MLKEM_INDCPA_PUBLICKEYBYTES + 2 as ::core::ffi::c_int * MLKEM_SYMBYTES;
pub const MLKEM_INDCCA_CIPHERTEXTBYTES: ::core::ffi::c_int = MLKEM_POLYVECCOMPRESSEDBYTES_DU
    + MLKEM_POLYCOMPRESSEDBYTES_DV;
#[inline]
unsafe extern "C" fn mlk_randombytes(
    mut ptr: *mut uint8_t,
    mut len: size_t,
) -> ::core::ffi::c_int {
    OQS_randombytes(ptr, len);
    return 0 as ::core::ffi::c_int;
}
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
unsafe extern "C" fn mlk_cast_int32_to_uint16(mut x: int32_t) -> uint16_t {
    return (x & UINT16_MAX as int32_t) as uint16_t;
}
#[inline]
unsafe extern "C" fn mlk_ct_cmask_nonzero_u16(mut x: uint16_t) -> uint16_t {
    let mut tmp: int32_t = mlk_value_barrier_i32(-(x as int32_t));
    tmp >>= 16 as ::core::ffi::c_int;
    return mlk_cast_int32_to_uint16(tmp);
}
#[inline]
unsafe extern "C" fn mlk_ct_cmask_nonzero_u8(mut x: uint8_t) -> uint8_t {
    let mut mask: uint16_t = mlk_ct_cmask_nonzero_u16(x as uint16_t);
    return (mask as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_ct_sel_uint8(
    mut a: uint8_t,
    mut b: uint8_t,
    mut cond: uint8_t,
) -> uint8_t {
    return (b as ::core::ffi::c_int
        ^ mlk_ct_cmask_nonzero_u8(cond) as ::core::ffi::c_int
            & (a as ::core::ffi::c_int ^ b as ::core::ffi::c_int)) as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_ct_memcmp(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
    len: size_t,
) -> uint8_t {
    let mut r: uint8_t = 0 as uint8_t;
    let mut s: uint8_t = 0 as uint8_t;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as size_t) < len {
        r = (r as ::core::ffi::c_int
            | *a.offset(i as isize) as ::core::ffi::c_int
                ^ *b.offset(i as isize) as ::core::ffi::c_int) as uint8_t;
        s = (s as ::core::ffi::c_int
            ^ (*a.offset(i as isize) as ::core::ffi::c_int
                ^ *b.offset(i as isize) as ::core::ffi::c_int)) as uint8_t;
        i = i.wrapping_add(1);
    }
    return (mlk_value_barrier_u8(
        (mlk_ct_cmask_nonzero_u8(r) as ::core::ffi::c_int ^ s as ::core::ffi::c_int)
            as uint8_t,
    ) as ::core::ffi::c_int ^ s as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn mlk_ct_cmov_zero(
    mut r: *mut uint8_t,
    mut x: *const uint8_t,
    mut len: size_t,
    mut b: uint8_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        *r.offset(i as isize) = mlk_ct_sel_uint8(
            *r.offset(i as isize),
            *x.offset(i as isize),
            b,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_check_pk(
    mut pk: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_p: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut p: *mut mlk_polyvec = &raw mut mlk_alloc_p as *mut mlk_polyvec;
    let mut mlk_alloc_p_reencoded: [uint8_t; 1152] = [0; 1152];
    let mut p_reencoded: *mut uint8_t = &raw mut mlk_alloc_p_reencoded as *mut uint8_t;
    if p.is_null() || p_reencoded.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_frombytes(p, pk as *const uint8_t);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(p);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tobytes(p_reencoded as *mut uint8_t, p);
        ret = if mlk_ct_memcmp(
            pk as *const uint8_t,
            p_reencoded,
            MLKEM_POLYVECBYTES as size_t,
        ) as ::core::ffi::c_int != 0
        {
            MLK_ERR_FAIL
        } else {
            0 as ::core::ffi::c_int
        };
    }
    mlk_zeroize(
        &raw mut mlk_alloc_p_reencoded as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 1152]>() as size_t,
    );
    p_reencoded = ::core::ptr::null_mut::<uint8_t>();
    mlk_zeroize(
        &raw mut mlk_alloc_p as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    p = ::core::ptr::null_mut::<mlk_polyvec>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_check_sk(
    mut sk: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_test: [uint8_t; 32] = [0; 32];
    let mut test: *mut uint8_t = &raw mut mlk_alloc_test as *mut uint8_t;
    if test.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        OQS_SHA3_sha3_256(
            test,
            sk.offset((3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int) as isize),
            (3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int) as size_t,
        );
        ret = if mlk_ct_memcmp(
            sk
                .offset(MLKEM_INDCCA_SECRETKEYBYTES as isize)
                .offset(-((2 as ::core::ffi::c_int * MLKEM_SYMBYTES) as isize)),
            test,
            MLKEM_SYMBYTES as size_t,
        ) as ::core::ffi::c_int != 0
        {
            MLK_ERR_FAIL
        } else {
            0 as ::core::ffi::c_int
        };
    }
    mlk_zeroize(
        &raw mut mlk_alloc_test as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    test = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
unsafe extern "C" fn mlk_check_pct(
    mut pk: *const uint8_t,
    mut sk: *const uint8_t,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_keypair_derand(
    mut pk: *mut uint8_t,
    mut sk: *mut uint8_t,
    mut coins: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_keypair_derand(
        pk,
        sk as *mut uint8_t,
        coins as *const uint8_t,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        memcpy(
            sk.offset(MLKEM_INDCPA_SECRETKEYBYTES as isize) as *mut ::core::ffi::c_void,
            pk as *const ::core::ffi::c_void,
            MLKEM_INDCCA_PUBLICKEYBYTES as size_t,
        );
        OQS_SHA3_sha3_256(
            sk
                .offset(
                    (3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int
                        + (3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
                )
                .offset(
                    -((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize),
                ),
            pk as *const uint8_t,
            (3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int) as size_t,
        );
        memcpy(
            sk
                .offset(MLKEM_INDCCA_SECRETKEYBYTES as isize)
                .offset(-(MLKEM_SYMBYTES as isize)) as *mut ::core::ffi::c_void,
            coins.offset(MLKEM_SYMBYTES as isize) as *const ::core::ffi::c_void,
            MLKEM_SYMBYTES as size_t,
        );
        ret = mlk_check_pct(pk as *const uint8_t, sk as *const uint8_t);
        ret != 0 as ::core::ffi::c_int;
    }
    if ret != 0 as ::core::ffi::c_int {
        mlk_zeroize(
            pk as *mut ::core::ffi::c_void,
            MLKEM_INDCCA_PUBLICKEYBYTES as size_t,
        );
        mlk_zeroize(
            sk as *mut ::core::ffi::c_void,
            MLKEM_INDCCA_SECRETKEYBYTES as size_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_keypair(
    mut pk: *mut uint8_t,
    mut sk: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_coins: [uint8_t; 64] = [0; 64];
    let mut coins: *mut uint8_t = &raw mut mlk_alloc_coins as *mut uint8_t;
    if coins.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else if mlk_randombytes(
        coins,
        (2 as ::core::ffi::c_int * MLKEM_SYMBYTES) as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        ret = MLK_ERR_RNG_FAIL;
    } else {
        ret = PQCP_MLKEM_NATIVE_MLKEM768_C_keypair_derand(
            pk,
            sk,
            coins as *const uint8_t,
        );
    }
    mlk_zeroize(
        &raw mut mlk_alloc_coins as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    coins = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_enc_derand(
    mut ct: *mut uint8_t,
    mut ss: *mut uint8_t,
    mut pk: *const uint8_t,
    mut coins: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_buf: [uint8_t; 64] = [0; 64];
    let mut buf: *mut uint8_t = &raw mut mlk_alloc_buf as *mut uint8_t;
    let mut mlk_alloc_kr: [uint8_t; 64] = [0; 64];
    let mut kr: *mut uint8_t = &raw mut mlk_alloc_kr as *mut uint8_t;
    if buf.is_null() || kr.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        ret = PQCP_MLKEM_NATIVE_MLKEM768_C_check_pk(pk);
        if !(ret != 0 as ::core::ffi::c_int) {
            memcpy(
                buf as *mut ::core::ffi::c_void,
                coins as *const ::core::ffi::c_void,
                MLKEM_SYMBYTES as size_t,
            );
            OQS_SHA3_sha3_256(
                buf.offset(32 as ::core::ffi::c_int as isize),
                pk as *const uint8_t,
                (3 as ::core::ffi::c_int * 384 as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int) as size_t,
            );
            OQS_SHA3_sha3_512(
                kr,
                buf,
                (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as size_t,
            );
            ret = PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_enc(
                ct,
                buf as *const uint8_t,
                pk,
                kr.offset(32 as ::core::ffi::c_int as isize) as *const uint8_t,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                memcpy(
                    ss as *mut ::core::ffi::c_void,
                    kr as *const ::core::ffi::c_void,
                    MLKEM_SYMBYTES as size_t,
                );
            }
        }
    }
    mlk_zeroize(
        &raw mut mlk_alloc_kr as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    kr = ::core::ptr::null_mut::<uint8_t>();
    mlk_zeroize(
        &raw mut mlk_alloc_buf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    buf = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_enc(
    mut ct: *mut uint8_t,
    mut ss: *mut uint8_t,
    mut pk: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_coins: [uint8_t; 32] = [0; 32];
    let mut coins: *mut uint8_t = &raw mut mlk_alloc_coins as *mut uint8_t;
    if coins.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else if mlk_randombytes(coins, MLKEM_SYMBYTES as size_t) != 0 as ::core::ffi::c_int
    {
        ret = MLK_ERR_RNG_FAIL;
    } else {
        ret = PQCP_MLKEM_NATIVE_MLKEM768_C_enc_derand(
            ct,
            ss,
            pk,
            coins as *const uint8_t,
        );
    }
    mlk_zeroize(
        &raw mut mlk_alloc_coins as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    coins = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_dec(
    mut ss: *mut uint8_t,
    mut ct: *const uint8_t,
    mut sk: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail: uint8_t = 0;
    let mut pk: *const uint8_t = sk.offset(MLKEM_INDCPA_SECRETKEYBYTES as isize);
    let mut mlk_alloc_buf: [uint8_t; 64] = [0; 64];
    let mut buf: *mut uint8_t = &raw mut mlk_alloc_buf as *mut uint8_t;
    let mut mlk_alloc_kr: [uint8_t; 64] = [0; 64];
    let mut kr: *mut uint8_t = &raw mut mlk_alloc_kr as *mut uint8_t;
    let mut mlk_alloc_tmp: [uint8_t; 1120] = [0; 1120];
    let mut tmp: *mut uint8_t = &raw mut mlk_alloc_tmp as *mut uint8_t;
    if buf.is_null() || kr.is_null() || tmp.is_null() {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        ret = PQCP_MLKEM_NATIVE_MLKEM768_C_check_sk(sk);
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_dec(
                buf as *mut uint8_t,
                ct,
                sk as *const uint8_t,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                memcpy(
                    buf.offset(MLKEM_SYMBYTES as isize) as *mut ::core::ffi::c_void,
                    sk
                        .offset(MLKEM_INDCCA_SECRETKEYBYTES as isize)
                        .offset(-((2 as ::core::ffi::c_int * MLKEM_SYMBYTES) as isize))
                        as *const ::core::ffi::c_void,
                    MLKEM_SYMBYTES as size_t,
                );
                OQS_SHA3_sha3_512(
                    kr,
                    buf,
                    (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as size_t,
                );
                ret = PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_enc(
                    tmp as *mut uint8_t,
                    buf as *const uint8_t,
                    pk as *const uint8_t,
                    kr.offset(32 as ::core::ffi::c_int as isize) as *const uint8_t,
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    fail = mlk_ct_memcmp(
                        ct as *const uint8_t,
                        tmp,
                        MLKEM_INDCCA_CIPHERTEXTBYTES as size_t,
                    );
                    memcpy(
                        tmp as *mut ::core::ffi::c_void,
                        sk
                            .offset(MLKEM_INDCCA_SECRETKEYBYTES as isize)
                            .offset(-(MLKEM_SYMBYTES as isize))
                            as *const ::core::ffi::c_void,
                        MLKEM_SYMBYTES as size_t,
                    );
                    memcpy(
                        tmp.offset(MLKEM_SYMBYTES as isize) as *mut ::core::ffi::c_void,
                        ct as *const ::core::ffi::c_void,
                        MLKEM_INDCCA_CIPHERTEXTBYTES as size_t,
                    );
                    OQS_SHA3_shake256(
                        ss as *mut uint8_t,
                        MLKEM_SYMBYTES as size_t,
                        tmp,
                        (32 as ::core::ffi::c_int
                            + (3 as ::core::ffi::c_int * 320 as ::core::ffi::c_int
                                + 128 as ::core::ffi::c_int)) as size_t,
                    );
                    mlk_ct_cmov_zero(
                        ss as *mut uint8_t,
                        kr,
                        MLKEM_SYMBYTES as size_t,
                        fail,
                    );
                }
            }
        }
    }
    mlk_zeroize(
        &raw mut mlk_alloc_tmp as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 1120]>() as size_t,
    );
    tmp = ::core::ptr::null_mut::<uint8_t>();
    mlk_zeroize(
        &raw mut mlk_alloc_kr as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    kr = ::core::ptr::null_mut::<uint8_t>();
    mlk_zeroize(
        &raw mut mlk_alloc_buf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    buf = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
pub const MLK_ERR_FAIL: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const MLK_ERR_OUT_OF_MEMORY: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const MLK_ERR_RNG_FAIL: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
