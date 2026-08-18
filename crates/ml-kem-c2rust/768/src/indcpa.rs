extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_reduce(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_add(r: *mut mlk_poly, b: *const mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_sub(r: *mut mlk_poly, b: *const mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_invntt_tomont(r: *mut mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_d4(
        r: *mut uint8_t,
        a: *const mlk_poly,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_d4(
        r: *mut mlk_poly,
        a: *const uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_frommsg(r: *mut mlk_poly, msg: *const uint8_t);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_tomsg(msg: *mut uint8_t, r: *const mlk_poly);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_compress_du(
        r: *mut uint8_t,
        a: *const mlk_polyvec,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_decompress_du(
        r: *mut mlk_polyvec,
        a: *const uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tobytes(
        r: *mut uint8_t,
        a: *const mlk_polyvec,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_frombytes(
        r: *mut mlk_polyvec,
        a: *const uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_ntt(r: *mut mlk_polyvec);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_invntt_tomont(r: *mut mlk_polyvec);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_basemul_acc_montgomery_cached(
        r: *mut mlk_poly,
        a: *const mlk_polyvec,
        b: *const mlk_polyvec,
        b_cache: *const mlk_polyvec_mulcache,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_mulcache_compute(
        x: *mut mlk_polyvec_mulcache,
        a: *const mlk_polyvec,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(r: *mut mlk_polyvec);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_add(
        r: *mut mlk_polyvec,
        b: *const mlk_polyvec,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tomont(r: *mut mlk_polyvec);
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_getnoise_eta1_4x(
        r0: *mut mlk_poly,
        r1: *mut mlk_poly,
        r2: *mut mlk_poly,
        r3: *mut mlk_poly,
        seed: *const uint8_t,
        nonce0: uint8_t,
        nonce1: uint8_t,
        nonce2: uint8_t,
        nonce3: uint8_t,
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_rej_uniform_x4(
        vec0: *mut mlk_poly,
        vec1: *mut mlk_poly,
        vec2: *mut mlk_poly,
        vec3: *mut mlk_poly,
        seed: *mut [uint8_t; 64],
    );
    fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_rej_uniform(
        entry: *mut mlk_poly,
        seed: *mut uint8_t,
    );
    fn OQS_SHA3_sha3_512(output: *mut uint8_t, input: *const uint8_t, inplen: size_t);
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
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
    pub vec: [mlk_poly; 3],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polyvec_PADDING: usize = ::core::mem::size_of::<mlk_polyvec>()
    - ::core::mem::size_of::<mlk_polyvec_Inner>();
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_polymat(pub mlk_polymat_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_polymat_Inner {
    pub vec: [mlk_polyvec; 3],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polymat_PADDING: usize = ::core::mem::size_of::<mlk_polymat>()
    - ::core::mem::size_of::<mlk_polymat_Inner>();
#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct mlk_polyvec_mulcache(pub mlk_polyvec_mulcache_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlk_polyvec_mulcache_Inner {
    pub vec: [mlk_poly_mulcache; 3],
}
#[allow(dead_code, non_upper_case_globals)]
const mlk_polyvec_mulcache_PADDING: usize = ::core::mem::size_of::<
    mlk_polyvec_mulcache,
>() - ::core::mem::size_of::<mlk_polyvec_mulcache_Inner>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MLKEM_K: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MLKEM_SYMBYTES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MLKEM_POLYBYTES: ::core::ffi::c_int = 384 as ::core::ffi::c_int;
pub const MLKEM_POLYVECBYTES: ::core::ffi::c_int = MLKEM_K * MLKEM_POLYBYTES;
pub const MLKEM_POLYCOMPRESSEDBYTES_D10: ::core::ffi::c_int = 320 as ::core::ffi::c_int;
pub const MLKEM_POLYCOMPRESSEDBYTES_DU: ::core::ffi::c_int = MLKEM_POLYCOMPRESSEDBYTES_D10;
pub const MLKEM_POLYVECCOMPRESSEDBYTES_DU: ::core::ffi::c_int = MLKEM_K
    * MLKEM_POLYCOMPRESSEDBYTES_DU;
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
unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_dv(
    mut r: *mut uint8_t,
    mut a: *const mlk_poly,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_d4(r, a);
}
#[inline]
unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_dv(
    mut r: *mut mlk_poly,
    mut a: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_d4(r, a);
}
unsafe extern "C" fn mlk_pack_pk(
    mut r: *mut uint8_t,
    mut pk: *const mlk_polyvec,
    mut seed: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tobytes(r as *mut uint8_t, pk);
    memcpy(
        r.offset(MLKEM_POLYVECBYTES as isize) as *mut ::core::ffi::c_void,
        seed as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
}
unsafe extern "C" fn mlk_unpack_pk(
    mut pk: *mut mlk_polyvec,
    mut seed: *mut uint8_t,
    mut packedpk: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_frombytes(pk, packedpk as *const uint8_t);
    memcpy(
        seed as *mut ::core::ffi::c_void,
        packedpk.offset(MLKEM_POLYVECBYTES as isize) as *const ::core::ffi::c_void,
        MLKEM_SYMBYTES as size_t,
    );
}
unsafe extern "C" fn mlk_pack_sk(mut r: *mut uint8_t, mut sk: *const mlk_polyvec) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tobytes(r, sk);
}
unsafe extern "C" fn mlk_unpack_sk(
    mut sk: *mut mlk_polyvec,
    mut packedsk: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_frombytes(sk, packedsk);
}
unsafe extern "C" fn mlk_pack_ciphertext(
    mut r: *mut uint8_t,
    mut b: *const mlk_polyvec,
    mut v: *mut mlk_poly,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_compress_du(r as *mut uint8_t, b);
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_compress_dv(
        r.offset(MLKEM_POLYVECCOMPRESSEDBYTES_DU as isize),
        v,
    );
}
unsafe extern "C" fn mlk_unpack_ciphertext(
    mut b: *mut mlk_polyvec,
    mut v: *mut mlk_poly,
    mut c: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_decompress_du(b, c as *const uint8_t);
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_decompress_dv(
        v,
        c.offset(MLKEM_POLYVECCOMPRESSEDBYTES_DU as isize),
    );
}
unsafe extern "C" fn mlk_polyvec_permute_bitrev_to_custom(mut v: *mut mlk_polyvec) {}
unsafe extern "C" fn mlk_polymat_permute_bitrev_to_custom(mut a: *mut mlk_polymat) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        mlk_polyvec_permute_bitrev_to_custom(
            (&raw mut (*a).0.vec as *mut mlk_polyvec).offset(i as isize)
                as *mut mlk_polyvec,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_gen_matrix(
    mut a: *mut mlk_polymat,
    mut seed: *const uint8_t,
    mut transposed: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut seed_ext: [[uint8_t; 64]; 4] = [[0; 64]; 4];
    j = 0 as ::core::ffi::c_uint;
    while j < 4 as ::core::ffi::c_uint {
        memcpy(
            &raw mut *(&raw mut seed_ext as *mut [uint8_t; 64]).offset(j as isize)
                as *mut uint8_t as *mut ::core::ffi::c_void,
            seed as *const ::core::ffi::c_void,
            MLKEM_SYMBYTES as size_t,
        );
        j = j.wrapping_add(1);
    }
    i = 0 as ::core::ffi::c_uint;
    while i
        < (MLKEM_K * MLKEM_K / 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
            as ::core::ffi::c_uint
    {
        j = 0 as ::core::ffi::c_uint;
        while j < 4 as ::core::ffi::c_uint {
            let mut x: uint8_t = 0;
            let mut y: uint8_t = 0;
            x = i.wrapping_add(j).wrapping_div(MLKEM_K as ::core::ffi::c_uint)
                as uint8_t;
            y = i.wrapping_add(j).wrapping_rem(MLKEM_K as ::core::ffi::c_uint)
                as uint8_t;
            if transposed != 0 {
                seed_ext[j
                    as usize][(MLKEM_SYMBYTES + 0 as ::core::ffi::c_int) as usize] = x;
                seed_ext[j
                    as usize][(MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as usize] = y;
            } else {
                seed_ext[j
                    as usize][(MLKEM_SYMBYTES + 0 as ::core::ffi::c_int) as usize] = y;
                seed_ext[j
                    as usize][(MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as usize] = x;
            }
            j = j.wrapping_add(1);
        }
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_rej_uniform_x4(
            (&raw mut (*(&raw mut (*a).0.vec as *mut mlk_polyvec)
                .offset(i.wrapping_div(MLKEM_K as ::core::ffi::c_uint) as isize))
                .0
                .vec as *mut mlk_poly)
                .offset(i.wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as isize)
                as *mut mlk_poly,
            (&raw mut (*(&raw mut (*a).0.vec as *mut mlk_polyvec)
                .offset(
                    i
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_div(MLKEM_K as ::core::ffi::c_uint) as isize,
                ))
                .0
                .vec as *mut mlk_poly)
                .offset(
                    i
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        .wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as isize,
                ) as *mut mlk_poly,
            (&raw mut (*(&raw mut (*a).0.vec as *mut mlk_polyvec)
                .offset(
                    i
                        .wrapping_add(2 as ::core::ffi::c_uint)
                        .wrapping_div(MLKEM_K as ::core::ffi::c_uint) as isize,
                ))
                .0
                .vec as *mut mlk_poly)
                .offset(
                    i
                        .wrapping_add(2 as ::core::ffi::c_uint)
                        .wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as isize,
                ) as *mut mlk_poly,
            (&raw mut (*(&raw mut (*a).0.vec as *mut mlk_polyvec)
                .offset(
                    i
                        .wrapping_add(3 as ::core::ffi::c_uint)
                        .wrapping_div(MLKEM_K as ::core::ffi::c_uint) as isize,
                ))
                .0
                .vec as *mut mlk_poly)
                .offset(
                    i
                        .wrapping_add(3 as ::core::ffi::c_uint)
                        .wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as isize,
                ) as *mut mlk_poly,
            &raw mut seed_ext as *mut [uint8_t; 64],
        );
        i = i.wrapping_add(4 as ::core::ffi::c_uint);
    }
    while i < (MLKEM_K * MLKEM_K) as ::core::ffi::c_uint {
        let mut x_0: uint8_t = 0;
        let mut y_0: uint8_t = 0;
        x_0 = i.wrapping_div(MLKEM_K as ::core::ffi::c_uint) as uint8_t;
        y_0 = i.wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as uint8_t;
        if transposed != 0 {
            seed_ext[0 as ::core::ffi::c_int
                as usize][(MLKEM_SYMBYTES + 0 as ::core::ffi::c_int) as usize] = x_0;
            seed_ext[0 as ::core::ffi::c_int
                as usize][(MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as usize] = y_0;
        } else {
            seed_ext[0 as ::core::ffi::c_int
                as usize][(MLKEM_SYMBYTES + 0 as ::core::ffi::c_int) as usize] = y_0;
            seed_ext[0 as ::core::ffi::c_int
                as usize][(MLKEM_SYMBYTES + 1 as ::core::ffi::c_int) as usize] = x_0;
        }
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_rej_uniform(
            (&raw mut (*(&raw mut (*a).0.vec as *mut mlk_polyvec)
                .offset(i.wrapping_div(MLKEM_K as ::core::ffi::c_uint) as isize))
                .0
                .vec as *mut mlk_poly)
                .offset(i.wrapping_rem(MLKEM_K as ::core::ffi::c_uint) as isize)
                as *mut mlk_poly,
            &raw mut *(&raw mut seed_ext as *mut [uint8_t; 64])
                .offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t,
        );
        i = i.wrapping_add(1);
    }
    mlk_polymat_permute_bitrev_to_custom(a);
    mlk_zeroize(
        &raw mut seed_ext as *mut [uint8_t; 64] as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[uint8_t; 64]; 4]>() as size_t,
    );
}
unsafe extern "C" fn mlk_matvec_mul(
    mut out: *mut mlk_polyvec,
    mut a: *const mlk_polymat,
    mut v: *const mlk_polyvec,
    mut vc: *const mlk_polyvec_mulcache,
) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < MLKEM_K as ::core::ffi::c_uint {
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_basemul_acc_montgomery_cached(
            (&raw mut (*out).0.vec as *mut mlk_poly).offset(i as isize) as *mut mlk_poly,
            (&raw const (*a).0.vec as *const mlk_polyvec).offset(i as isize)
                as *const mlk_polyvec,
            v,
            vc,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mlk_keypair_getnoise_eta1(
    mut pv: *mut mlk_polyvec,
    mut e: *mut mlk_polyvec,
    mut seed: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_getnoise_eta1_4x(
        (&raw mut (*pv).0.vec as *mut mlk_poly).offset(0 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*pv).0.vec as *mut mlk_poly).offset(1 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*pv).0.vec as *mut mlk_poly).offset(2 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        ::core::ptr::null_mut::<mlk_poly>(),
        seed,
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        0xff as uint8_t,
    );
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_getnoise_eta1_4x(
        (&raw mut (*e).0.vec as *mut mlk_poly).offset(0 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*e).0.vec as *mut mlk_poly).offset(1 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*e).0.vec as *mut mlk_poly).offset(2 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        ::core::ptr::null_mut::<mlk_poly>(),
        seed,
        3 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        0xff as uint8_t,
    );
}
unsafe extern "C" fn mlk_enc_getnoise_eta1_eta2(
    mut sp: *mut mlk_polyvec,
    mut ep: *mut mlk_polyvec,
    mut epp: *mut mlk_poly,
    mut coins: *const uint8_t,
) {
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_getnoise_eta1_4x(
        (&raw mut (*sp).0.vec as *mut mlk_poly).offset(0 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*sp).0.vec as *mut mlk_poly).offset(1 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*sp).0.vec as *mut mlk_poly).offset(2 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        ::core::ptr::null_mut::<mlk_poly>(),
        coins,
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        0xff as uint8_t,
    );
    PQCP_MLKEM_NATIVE_MLKEM768_C_poly_getnoise_eta1_4x(
        (&raw mut (*ep).0.vec as *mut mlk_poly).offset(0 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*ep).0.vec as *mut mlk_poly).offset(1 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        (&raw mut (*ep).0.vec as *mut mlk_poly).offset(2 as ::core::ffi::c_int as isize)
            as *mut mlk_poly,
        epp,
        coins,
        3 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_keypair_derand(
    mut pk: *mut uint8_t,
    mut sk: *mut uint8_t,
    mut coins: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut publicseed: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut noiseseed: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut mlk_alloc_buf: [uint8_t; 64] = [0; 64];
    let mut buf: *mut uint8_t = &raw mut mlk_alloc_buf as *mut uint8_t;
    let mut mlk_alloc_coins_with_domain_separator: [uint8_t; 33] = [0; 33];
    let mut coins_with_domain_separator: *mut uint8_t = &raw mut mlk_alloc_coins_with_domain_separator
        as *mut uint8_t;
    let mut mlk_alloc_a: [mlk_polymat; 1] = [mlk_polymat(mlk_polymat_Inner {
        vec: [mlk_polyvec(mlk_polyvec_Inner {
            vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
        }); 3],
    }); 1];
    let mut a: *mut mlk_polymat = &raw mut mlk_alloc_a as *mut mlk_polymat;
    let mut mlk_alloc_e: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut e: *mut mlk_polyvec = &raw mut mlk_alloc_e as *mut mlk_polyvec;
    let mut mlk_alloc_pkpv: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut pkpv: *mut mlk_polyvec = &raw mut mlk_alloc_pkpv as *mut mlk_polyvec;
    let mut mlk_alloc_skpv: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut skpv: *mut mlk_polyvec = &raw mut mlk_alloc_skpv as *mut mlk_polyvec;
    let mut mlk_alloc_skpv_cache: [mlk_polyvec_mulcache; 1] = [mlk_polyvec_mulcache(mlk_polyvec_mulcache_Inner {
        vec: [mlk_poly_mulcache(mlk_poly_mulcache_Inner {
            coeffs: [0; 128],
        }); 3],
    }); 1];
    let mut skpv_cache: *mut mlk_polyvec_mulcache = &raw mut mlk_alloc_skpv_cache
        as *mut mlk_polyvec_mulcache;
    if buf.is_null() || coins_with_domain_separator.is_null() || a.is_null()
        || e.is_null() || pkpv.is_null() || skpv.is_null() || skpv_cache.is_null()
    {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        publicseed = buf;
        noiseseed = buf.offset(MLKEM_SYMBYTES as isize);
        memcpy(
            coins_with_domain_separator as *mut ::core::ffi::c_void,
            coins as *const ::core::ffi::c_void,
            MLKEM_SYMBYTES as size_t,
        );
        *coins_with_domain_separator.offset(MLKEM_SYMBYTES as isize) = MLKEM_K
            as uint8_t;
        OQS_SHA3_sha3_512(
            buf,
            coins_with_domain_separator,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        );
        PQCP_MLKEM_NATIVE_MLKEM768_C_gen_matrix(
            a,
            publicseed as *const uint8_t,
            0 as ::core::ffi::c_int,
        );
        mlk_keypair_getnoise_eta1(skpv, e, noiseseed as *const uint8_t);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_ntt(skpv);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_ntt(e);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_mulcache_compute(skpv_cache, skpv);
        mlk_matvec_mul(pkpv, a, skpv, skpv_cache);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_tomont(pkpv);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_add(pkpv, e);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(pkpv);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(skpv);
        mlk_pack_sk(sk, skpv);
        mlk_pack_pk(pk, pkpv, publicseed as *const uint8_t);
    }
    mlk_zeroize(
        &raw mut mlk_alloc_skpv_cache as *mut mlk_polyvec_mulcache
            as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec_mulcache; 1]>() as size_t,
    );
    skpv_cache = ::core::ptr::null_mut::<mlk_polyvec_mulcache>();
    mlk_zeroize(
        &raw mut mlk_alloc_skpv as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    skpv = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_pkpv as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    pkpv = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_e as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    e = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_a as *mut mlk_polymat as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polymat; 1]>() as size_t,
    );
    a = ::core::ptr::null_mut::<mlk_polymat>();
    mlk_zeroize(
        &raw mut mlk_alloc_coins_with_domain_separator as *mut uint8_t
            as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 33]>() as size_t,
    );
    coins_with_domain_separator = ::core::ptr::null_mut::<uint8_t>();
    mlk_zeroize(
        &raw mut mlk_alloc_buf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    buf = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_enc(
    mut c: *mut uint8_t,
    mut m: *const uint8_t,
    mut pk: *const uint8_t,
    mut coins: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_seed: [uint8_t; 32] = [0; 32];
    let mut seed: *mut uint8_t = &raw mut mlk_alloc_seed as *mut uint8_t;
    let mut mlk_alloc_at: [mlk_polymat; 1] = [mlk_polymat(mlk_polymat_Inner {
        vec: [mlk_polyvec(mlk_polyvec_Inner {
            vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
        }); 3],
    }); 1];
    let mut at: *mut mlk_polymat = &raw mut mlk_alloc_at as *mut mlk_polymat;
    let mut mlk_alloc_sp: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut sp: *mut mlk_polyvec = &raw mut mlk_alloc_sp as *mut mlk_polyvec;
    let mut mlk_alloc_pkpv: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut pkpv: *mut mlk_polyvec = &raw mut mlk_alloc_pkpv as *mut mlk_polyvec;
    let mut mlk_alloc_ep: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut ep: *mut mlk_polyvec = &raw mut mlk_alloc_ep as *mut mlk_polyvec;
    let mut mlk_alloc_b: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut b: *mut mlk_polyvec = &raw mut mlk_alloc_b as *mut mlk_polyvec;
    let mut mlk_alloc_v: [mlk_poly; 1] = [mlk_poly(mlk_poly_Inner {
        coeffs: [0; 256],
    }); 1];
    let mut v: *mut mlk_poly = &raw mut mlk_alloc_v as *mut mlk_poly;
    let mut mlk_alloc_k: [mlk_poly; 1] = [mlk_poly(mlk_poly_Inner {
        coeffs: [0; 256],
    }); 1];
    let mut k: *mut mlk_poly = &raw mut mlk_alloc_k as *mut mlk_poly;
    let mut mlk_alloc_epp: [mlk_poly; 1] = [mlk_poly(mlk_poly_Inner {
        coeffs: [0; 256],
    }); 1];
    let mut epp: *mut mlk_poly = &raw mut mlk_alloc_epp as *mut mlk_poly;
    let mut mlk_alloc_sp_cache: [mlk_polyvec_mulcache; 1] = [mlk_polyvec_mulcache(mlk_polyvec_mulcache_Inner {
        vec: [mlk_poly_mulcache(mlk_poly_mulcache_Inner {
            coeffs: [0; 128],
        }); 3],
    }); 1];
    let mut sp_cache: *mut mlk_polyvec_mulcache = &raw mut mlk_alloc_sp_cache
        as *mut mlk_polyvec_mulcache;
    if seed.is_null() || at.is_null() || sp.is_null() || pkpv.is_null() || ep.is_null()
        || b.is_null() || v.is_null() || k.is_null() || epp.is_null()
        || sp_cache.is_null()
    {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        mlk_unpack_pk(pkpv, seed as *mut uint8_t, pk);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_frommsg(k, m);
        PQCP_MLKEM_NATIVE_MLKEM768_C_gen_matrix(
            at,
            seed as *const uint8_t,
            1 as ::core::ffi::c_int,
        );
        mlk_enc_getnoise_eta1_eta2(sp, ep, epp, coins);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_ntt(sp);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_mulcache_compute(sp_cache, sp);
        mlk_matvec_mul(b, at, sp, sp_cache);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_basemul_acc_montgomery_cached(
            v,
            pkpv,
            sp,
            sp_cache,
        );
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_invntt_tomont(b);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_invntt_tomont(v);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_add(b, ep);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_add(v, epp);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_add(v, k);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_reduce(b);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_reduce(v);
        mlk_pack_ciphertext(c, b, v);
    }
    mlk_zeroize(
        &raw mut mlk_alloc_sp_cache as *mut mlk_polyvec_mulcache
            as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec_mulcache; 1]>() as size_t,
    );
    sp_cache = ::core::ptr::null_mut::<mlk_polyvec_mulcache>();
    mlk_zeroize(
        &raw mut mlk_alloc_epp as *mut mlk_poly as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_poly; 1]>() as size_t,
    );
    epp = ::core::ptr::null_mut::<mlk_poly>();
    mlk_zeroize(
        &raw mut mlk_alloc_k as *mut mlk_poly as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_poly; 1]>() as size_t,
    );
    k = ::core::ptr::null_mut::<mlk_poly>();
    mlk_zeroize(
        &raw mut mlk_alloc_v as *mut mlk_poly as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_poly; 1]>() as size_t,
    );
    v = ::core::ptr::null_mut::<mlk_poly>();
    mlk_zeroize(
        &raw mut mlk_alloc_b as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    b = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_ep as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    ep = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_pkpv as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    pkpv = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_sp as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    sp = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_at as *mut mlk_polymat as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polymat; 1]>() as size_t,
    );
    at = ::core::ptr::null_mut::<mlk_polymat>();
    mlk_zeroize(
        &raw mut mlk_alloc_seed as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    seed = ::core::ptr::null_mut::<uint8_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PQCP_MLKEM_NATIVE_MLKEM768_C_indcpa_dec(
    mut m: *mut uint8_t,
    mut c: *const uint8_t,
    mut sk: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mlk_alloc_b: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut b: *mut mlk_polyvec = &raw mut mlk_alloc_b as *mut mlk_polyvec;
    let mut mlk_alloc_skpv: [mlk_polyvec; 1] = [mlk_polyvec(mlk_polyvec_Inner {
        vec: [mlk_poly(mlk_poly_Inner { coeffs: [0; 256] }); 3],
    }); 1];
    let mut skpv: *mut mlk_polyvec = &raw mut mlk_alloc_skpv as *mut mlk_polyvec;
    let mut mlk_alloc_v: [mlk_poly; 1] = [mlk_poly(mlk_poly_Inner {
        coeffs: [0; 256],
    }); 1];
    let mut v: *mut mlk_poly = &raw mut mlk_alloc_v as *mut mlk_poly;
    let mut mlk_alloc_sb: [mlk_poly; 1] = [mlk_poly(mlk_poly_Inner {
        coeffs: [0; 256],
    }); 1];
    let mut sb: *mut mlk_poly = &raw mut mlk_alloc_sb as *mut mlk_poly;
    let mut mlk_alloc_b_cache: [mlk_polyvec_mulcache; 1] = [mlk_polyvec_mulcache(mlk_polyvec_mulcache_Inner {
        vec: [mlk_poly_mulcache(mlk_poly_mulcache_Inner {
            coeffs: [0; 128],
        }); 3],
    }); 1];
    let mut b_cache: *mut mlk_polyvec_mulcache = &raw mut mlk_alloc_b_cache
        as *mut mlk_polyvec_mulcache;
    if b.is_null() || skpv.is_null() || v.is_null() || sb.is_null() || b_cache.is_null()
    {
        ret = MLK_ERR_OUT_OF_MEMORY;
    } else {
        mlk_unpack_ciphertext(b, v, c);
        mlk_unpack_sk(skpv, sk);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_ntt(b);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_mulcache_compute(b_cache, b);
        PQCP_MLKEM_NATIVE_MLKEM768_C_polyvec_basemul_acc_montgomery_cached(
            sb,
            skpv,
            b,
            b_cache,
        );
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_invntt_tomont(sb);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_sub(v, sb);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_reduce(v);
        PQCP_MLKEM_NATIVE_MLKEM768_C_poly_tomsg(m, v);
    }
    mlk_zeroize(
        &raw mut mlk_alloc_b_cache as *mut mlk_polyvec_mulcache
            as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec_mulcache; 1]>() as size_t,
    );
    b_cache = ::core::ptr::null_mut::<mlk_polyvec_mulcache>();
    mlk_zeroize(
        &raw mut mlk_alloc_sb as *mut mlk_poly as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_poly; 1]>() as size_t,
    );
    sb = ::core::ptr::null_mut::<mlk_poly>();
    mlk_zeroize(
        &raw mut mlk_alloc_v as *mut mlk_poly as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_poly; 1]>() as size_t,
    );
    v = ::core::ptr::null_mut::<mlk_poly>();
    mlk_zeroize(
        &raw mut mlk_alloc_skpv as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    skpv = ::core::ptr::null_mut::<mlk_polyvec>();
    mlk_zeroize(
        &raw mut mlk_alloc_b as *mut mlk_polyvec as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[mlk_polyvec; 1]>() as size_t,
    );
    b = ::core::ptr::null_mut::<mlk_polyvec>();
    return ret;
}
pub const MLK_ERR_OUT_OF_MEMORY: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
