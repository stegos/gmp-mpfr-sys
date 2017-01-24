// Copyright © 2017 University of Malta

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <http://www.gnu.org/licenses/>.

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

pub type mp_limb_t = c_ulong;
pub type mp_limb_signed_t = c_long;
pub type mp_bitcnt_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];

pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = c_long;
pub type mp_exp_t = c_long;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpq_struct {
    pub _mp_num: __mpz_struct,
    pub _mp_den: __mpz_struct,
}
pub type mpq_t = [__mpq_struct; 1];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpf_struct {
    pub _mp_prec: c_int,
    pub _mp_size: c_int,
    pub _mp_exp: mp_exp_t,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpf_t = [__mpf_struct; 1];

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum gmp_randalg_t {
    GMP_RAND_ALG_DEFAULT = 0,
}
pub const GMP_RAND_ALG_LC: gmp_randalg_t = gmp_randalg_t::GMP_RAND_ALG_DEFAULT;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __gmp_randstate_struct {
    pub _mp_seed: mpz_t,
    pub _mp_alg: gmp_randalg_t,
    pub _mp_algdata: __gmp_randstate_struct_union,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __gmp_randstate_struct_union {
    pub _mp_lc: *mut c_void,
}
pub type gmp_randstate_t = [__gmp_randstate_struct; 1];

pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
pub type mpf_srcptr = *const __mpf_struct;
pub type mpf_ptr = *mut __mpf_struct;
pub type mpq_srcptr = *const __mpq_struct;
pub type mpq_ptr = *mut __mpq_struct;

#[inline]
pub unsafe fn mpq_numref(Q: mpq_ptr) -> mpz_ptr {
    (&mut (*Q)._mp_num) as mpz_ptr
}
#[inline]
pub unsafe fn mpq_denref(Q: mpq_ptr) -> mpz_ptr {
    (&mut (*Q)._mp_den) as mpz_ptr
}

pub const GMP_ERROR_NONE: u32 = 0;
pub const GMP_ERROR_UNSUPPORTED_ARGUMENT: u32 = 1;
pub const GMP_ERROR_DIVISION_BY_ZERO: u32 = 2;
pub const GMP_ERROR_SQRT_OF_NEGATIVE: u32 = 4;
pub const GMP_ERROR_INVALID_ARGUMENT: u32 = 8;

pub const __GNU_MP_VERSION: u32 = 6;
pub const __GNU_MP_VERSION_MINOR: u32 = 1;
pub const __GNU_MP_VERSION_PATCHLEVEL: u32 = 2;
pub const __GNU_MP_RELEASE: u32 = 60102;

#[link(name = "gmp", kind = "static")]
extern "C" {
    pub static __gmp_bits_per_limb: c_int;
    pub static mut __gmp_errno: c_int;
    pub static __gmp_version: *const c_char;
}
#[link(name = "gmp", kind = "static")]
extern "C" {
    pub fn __gmp_set_memory_functions(
        arg1: Option<extern "C" fn(arg1: usize) -> *mut c_void>,
        arg2: Option<unsafe extern "C" fn(arg1: *mut c_void,
                                          arg2: usize,
                                          arg3: usize)
                                          -> *mut c_void>,
        arg3: Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: usize)>
    );
    pub fn __gmp_get_memory_functions(
        arg1: *mut Option<extern "C" fn(arg1: usize) -> *mut c_void>,
        arg2: *mut Option<unsafe extern "C" fn(arg1: *mut c_void,
                                               arg2: usize,
                                               arg3: usize)
                                               -> *mut c_void>,
        arg3: *mut Option<unsafe extern "C" fn(arg1: *mut c_void,
                                               arg2: usize)>);

    // Random number routines.

    pub fn __gmp_randinit(arg1: gmp_randstate_t, arg2: gmp_randalg_t, ...);
    pub fn __gmp_randinit_default(arg1: gmp_randstate_t);
    pub fn __gmp_randinit_lc_2exp(arg1: gmp_randstate_t,
                                  arg2: mpz_srcptr,
                                  arg3: c_ulong,
                                  arg4: mp_bitcnt_t);
    pub fn __gmp_randinit_lc_2exp_size(arg1: gmp_randstate_t,
                                       arg2: mp_bitcnt_t)
                                       -> c_int;
    pub fn __gmp_randinit_mt(arg1: gmp_randstate_t);
    pub fn __gmp_randinit_set(arg1: gmp_randstate_t,
                              arg2: *const __gmp_randstate_struct);
    pub fn __gmp_randseed(arg1: gmp_randstate_t, arg2: mpz_srcptr);
    pub fn __gmp_randseed_ui(arg1: gmp_randstate_t, arg2: c_ulong);
    pub fn __gmp_randclear(arg1: gmp_randstate_t);
    pub fn __gmp_urandomb_ui(arg1: gmp_randstate_t, arg2: c_ulong) -> c_ulong;
    pub fn __gmp_urandomm_ui(arg1: gmp_randstate_t, arg2: c_ulong) -> c_ulong;

    // Formatted output routines.

    pub fn __gmp_asprintf(arg1: *mut *mut c_char,
                          arg2: *const c_char,
                          ...)
                          -> c_int;
    pub fn __gmp_printf(arg1: *const c_char, ...) -> c_int;
    pub fn __gmp_snprintf(arg1: *mut c_char,
                          arg2: usize,
                          arg3: *const c_char,
                          ...)
                          -> c_int;
    pub fn __gmp_sprintf(arg1: *mut c_char, arg2: *const c_char, ...) -> c_int;

    // Formatted input routines.

    pub fn __gmp_scanf(arg1: *const c_char, ...) -> c_int;
    pub fn __gmp_sscanf(arg1: *const c_char,
                        arg2: *const c_char,
                        ...)
                        -> c_int;

    // Integer (i.e. Z) routines.

    pub fn __gmpz_realloc(arg1: mpz_ptr, arg2: mp_size_t) -> *mut c_void;
    pub fn __gmpz_abs(__gmp_w: mpz_ptr, __gmp_u: mpz_srcptr);
    pub fn __gmpz_add(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_add_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_addmul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_addmul_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_and(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_array_init(arg1: mpz_ptr, arg2: mp_size_t, arg3: mp_size_t);
    pub fn __gmpz_bin_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_bin_uiui(arg1: mpz_ptr, arg2: c_ulong, arg3: c_ulong);
    pub fn __gmpz_cdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_cdiv_q_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_cdiv_q_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_cdiv_qr(arg1: mpz_ptr,
                          arg2: mpz_ptr,
                          arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_cdiv_qr_ui(arg1: mpz_ptr,
                             arg2: mpz_ptr,
                             arg3: mpz_srcptr,
                             arg4: c_ulong)
                             -> c_ulong;
    pub fn __gmpz_cdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_cdiv_r_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_cdiv_r_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_cdiv_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_ulong;
    pub fn __gmpz_clear(arg1: mpz_ptr);
    pub fn __gmpz_clears(arg1: mpz_ptr, ...);
    pub fn __gmpz_clrbit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_cmp(arg1: mpz_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_cmp_d(arg1: mpz_srcptr, arg2: f64) -> c_int;
    pub fn __gmpz_cmp_si(arg1: mpz_srcptr, arg2: c_long) -> c_int;
    pub fn __gmpz_cmp_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_int;
    pub fn __gmpz_cmpabs(arg1: mpz_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_cmpabs_d(arg1: mpz_srcptr, arg2: f64) -> c_int;
    pub fn __gmpz_cmpabs_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_int;
    pub fn __gmpz_com(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_combit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_congruent_p(arg1: mpz_srcptr,
                              arg2: mpz_srcptr,
                              arg3: mpz_srcptr)
                              -> c_int;
    pub fn __gmpz_congruent_2exp_p(arg1: mpz_srcptr,
                                   arg2: mpz_srcptr,
                                   arg3: mp_bitcnt_t)
                                   -> c_int;
    pub fn __gmpz_congruent_ui_p(arg1: mpz_srcptr,
                                 arg2: c_ulong,
                                 arg3: c_ulong)
                                 -> c_int;
    pub fn __gmpz_divexact(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_divexact_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_divisible_p(arg1: mpz_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_divisible_ui_p(arg1: mpz_srcptr, arg2: c_ulong) -> c_int;
    pub fn __gmpz_divisible_2exp_p(arg1: mpz_srcptr,
                                   arg2: mp_bitcnt_t)
                                   -> c_int;
    pub fn __gmpz_dump(arg1: mpz_srcptr);
    pub fn __gmpz_export(arg1: *mut c_void,
                         arg2: *mut usize,
                         arg3: c_int,
                         arg4: usize,
                         arg5: c_int,
                         arg6: usize,
                         arg7: mpz_srcptr)
                         -> *mut c_void;
    pub fn __gmpz_fac_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_2fac_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_mfac_uiui(arg1: mpz_ptr, arg2: c_ulong, arg3: c_ulong);
    pub fn __gmpz_primorial_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_fdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_fdiv_q_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_fdiv_q_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_fdiv_qr(arg1: mpz_ptr,
                          arg2: mpz_ptr,
                          arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_fdiv_qr_ui(arg1: mpz_ptr,
                             arg2: mpz_ptr,
                             arg3: mpz_srcptr,
                             arg4: c_ulong)
                             -> c_ulong;
    pub fn __gmpz_fdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_fdiv_r_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_fdiv_r_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_fdiv_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_ulong;
    pub fn __gmpz_fib_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_fib2_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: c_ulong);
    pub fn __gmpz_fits_sint_p(arg1: mpz_srcptr) -> c_int;
    pub fn __gmpz_fits_slong_p(arg1: mpz_srcptr) -> c_int;
    pub fn __gmpz_fits_sshort_p(arg1: mpz_srcptr) -> c_int;
    pub fn __gmpz_fits_uint_p(__gmp_z: mpz_srcptr) -> c_int;
    pub fn __gmpz_fits_ulong_p(__gmp_z: mpz_srcptr) -> c_int;
    pub fn __gmpz_fits_ushort_p(__gmp_z: mpz_srcptr) -> c_int;
    pub fn __gmpz_gcd(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_gcd_ui(arg1: mpz_ptr,
                         arg2: mpz_srcptr,
                         arg3: c_ulong)
                         -> c_ulong;
    pub fn __gmpz_gcdext(arg1: mpz_ptr,
                         arg2: mpz_ptr,
                         arg3: mpz_ptr,
                         arg4: mpz_srcptr,
                         arg5: mpz_srcptr);
    pub fn __gmpz_get_d(arg1: mpz_srcptr) -> f64;
    pub fn __gmpz_get_d_2exp(arg1: *mut c_long, arg2: mpz_srcptr) -> f64;
    pub fn __gmpz_get_si(arg1: mpz_srcptr) -> c_long;
    pub fn __gmpz_get_str(arg1: *mut c_char,
                          arg2: c_int,
                          arg3: mpz_srcptr)
                          -> *mut c_char;
    pub fn __gmpz_get_ui(__gmp_z: mpz_srcptr) -> c_ulong;
    pub fn __gmpz_getlimbn(__gmp_z: mpz_srcptr,
                           __gmp_n: mp_size_t)
                           -> mp_limb_t;
    pub fn __gmpz_hamdist(arg1: mpz_srcptr, arg2: mpz_srcptr) -> mp_bitcnt_t;
    pub fn __gmpz_import(arg1: mpz_ptr,
                         arg2: usize,
                         arg3: c_int,
                         arg4: usize,
                         arg5: c_int,
                         arg6: usize,
                         arg7: *const c_void);
    pub fn __gmpz_init(arg1: mpz_ptr);
    pub fn __gmpz_init2(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_inits(arg1: mpz_ptr, ...);
    pub fn __gmpz_init_set(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_init_set_d(arg1: mpz_ptr, arg2: f64);
    pub fn __gmpz_init_set_si(arg1: mpz_ptr, arg2: c_long);
    pub fn __gmpz_init_set_str(arg1: mpz_ptr,
                               arg2: *const c_char,
                               arg3: c_int)
                               -> c_int;
    pub fn __gmpz_init_set_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_invert(arg1: mpz_ptr,
                         arg2: mpz_srcptr,
                         arg3: mpz_srcptr)
                         -> c_int;
    pub fn __gmpz_ior(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_jacobi(arg1: mpz_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_kronecker_si(arg1: mpz_srcptr, arg2: c_long) -> c_int;
    pub fn __gmpz_kronecker_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_int;
    pub fn __gmpz_si_kronecker(arg1: c_long, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_ui_kronecker(arg1: c_ulong, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpz_lcm(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_lcm_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_lucnum_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_lucnum2_ui(arg1: mpz_ptr, arg2: mpz_ptr, arg3: c_ulong);
    pub fn __gmpz_millerrabin(arg1: mpz_srcptr, arg2: c_int) -> c_int;
    pub fn __gmpz_mod(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_mul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_mul_2exp(arg1: mpz_ptr,
                           arg2: mpz_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_mul_si(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_long);
    pub fn __gmpz_mul_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_neg(__gmp_w: mpz_ptr, __gmp_u: mpz_srcptr);
    pub fn __gmpz_nextprime(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_perfect_power_p(arg1: mpz_srcptr) -> c_int;
    pub fn __gmpz_perfect_square_p(__gmp_a: mpz_srcptr) -> c_int;
    pub fn __gmpz_popcount(__gmp_u: mpz_srcptr) -> mp_bitcnt_t;
    pub fn __gmpz_pow_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_powm(arg1: mpz_ptr,
                       arg2: mpz_srcptr,
                       arg3: mpz_srcptr,
                       arg4: mpz_srcptr);
    pub fn __gmpz_powm_sec(arg1: mpz_ptr,
                           arg2: mpz_srcptr,
                           arg3: mpz_srcptr,
                           arg4: mpz_srcptr);
    pub fn __gmpz_powm_ui(arg1: mpz_ptr,
                          arg2: mpz_srcptr,
                          arg3: c_ulong,
                          arg4: mpz_srcptr);
    pub fn __gmpz_probab_prime_p(arg1: mpz_srcptr, arg2: c_int) -> c_int;
    pub fn __gmpz_random(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_random2(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_realloc2(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_remove(arg1: mpz_ptr,
                         arg2: mpz_srcptr,
                         arg3: mpz_srcptr)
                         -> mp_bitcnt_t;
    pub fn __gmpz_root(arg1: mpz_ptr,
                       arg2: mpz_srcptr,
                       arg3: c_ulong)
                       -> c_int;
    pub fn __gmpz_rootrem(arg1: mpz_ptr,
                          arg2: mpz_ptr,
                          arg3: mpz_srcptr,
                          arg4: c_ulong);
    pub fn __gmpz_rrandomb(arg1: mpz_ptr,
                           arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_scan0(arg1: mpz_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpz_scan1(arg1: mpz_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpz_set(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_set_d(arg1: mpz_ptr, arg2: f64);
    pub fn __gmpz_set_f(arg1: mpz_ptr, arg2: mpf_srcptr);
    pub fn __gmpz_set_q(__gmp_w: mpz_ptr, __gmp_u: mpq_srcptr);
    pub fn __gmpz_set_si(arg1: mpz_ptr, arg2: c_long);
    pub fn __gmpz_set_str(arg1: mpz_ptr,
                          arg2: *const c_char,
                          arg3: c_int)
                          -> c_int;
    pub fn __gmpz_set_ui(arg1: mpz_ptr, arg2: c_ulong);
    pub fn __gmpz_setbit(arg1: mpz_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpz_size(__gmp_z: mpz_srcptr) -> usize;
    pub fn __gmpz_sizeinbase(arg1: mpz_srcptr, arg2: c_int) -> usize;
    pub fn __gmpz_sqrt(arg1: mpz_ptr, arg2: mpz_srcptr);
    pub fn __gmpz_sqrtrem(arg1: mpz_ptr, arg2: mpz_ptr, arg3: mpz_srcptr);
    pub fn __gmpz_sub(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_sub_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_ui_sub(arg1: mpz_ptr, arg2: c_ulong, arg3: mpz_srcptr);
    pub fn __gmpz_submul(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_submul_ui(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: c_ulong);
    pub fn __gmpz_swap(arg1: mpz_ptr, arg2: mpz_ptr);
    pub fn __gmpz_tdiv_ui(arg1: mpz_srcptr, arg2: c_ulong) -> c_ulong;
    pub fn __gmpz_tdiv_q(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_tdiv_q_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_tdiv_q_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_tdiv_qr(arg1: mpz_ptr,
                          arg2: mpz_ptr,
                          arg3: mpz_srcptr,
                          arg4: mpz_srcptr);
    pub fn __gmpz_tdiv_qr_ui(arg1: mpz_ptr,
                             arg2: mpz_ptr,
                             arg3: mpz_srcptr,
                             arg4: c_ulong)
                             -> c_ulong;
    pub fn __gmpz_tdiv_r(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_tdiv_r_2exp(arg1: mpz_ptr,
                              arg2: mpz_srcptr,
                              arg3: mp_bitcnt_t);
    pub fn __gmpz_tdiv_r_ui(arg1: mpz_ptr,
                            arg2: mpz_srcptr,
                            arg3: c_ulong)
                            -> c_ulong;
    pub fn __gmpz_tstbit(arg1: mpz_srcptr, arg2: mp_bitcnt_t) -> c_int;
    pub fn __gmpz_ui_pow_ui(arg1: mpz_ptr, arg2: c_ulong, arg3: c_ulong);
    pub fn __gmpz_urandomb(arg1: mpz_ptr,
                           arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);
    pub fn __gmpz_urandomm(arg1: mpz_ptr,
                           arg2: gmp_randstate_t,
                           arg3: mpz_srcptr);
    pub fn __gmpz_xor(arg1: mpz_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr);
    pub fn __gmpz_limbs_read(arg1: mpz_srcptr) -> mp_srcptr;
    pub fn __gmpz_limbs_write(arg1: mpz_ptr, arg2: mp_size_t) -> mp_ptr;
    pub fn __gmpz_limbs_modify(arg1: mpz_ptr, arg2: mp_size_t) -> mp_ptr;
    pub fn __gmpz_limbs_finish(arg1: mpz_ptr, arg2: mp_size_t);
    pub fn __gmpz_roinit_n(arg1: mpz_ptr,
                           arg2: mp_srcptr,
                           arg3: mp_size_t)
                           -> mpz_srcptr;

    // Rational (i.e. Q) routines.

    pub fn __gmpq_abs(__gmp_w: mpq_ptr, __gmp_u: mpq_srcptr);
    pub fn __gmpq_add(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_canonicalize(arg1: mpq_ptr);
    pub fn __gmpq_clear(arg1: mpq_ptr);
    pub fn __gmpq_clears(arg1: mpq_ptr, ...);
    pub fn __gmpq_cmp(arg1: mpq_srcptr, arg2: mpq_srcptr) -> c_int;
    pub fn __gmpq_cmp_si(arg1: mpq_srcptr,
                         arg2: c_long,
                         arg3: c_ulong)
                         -> c_int;
    pub fn __gmpq_cmp_ui(arg1: mpq_srcptr,
                         arg2: c_ulong,
                         arg3: c_ulong)
                         -> c_int;
    pub fn __gmpq_cmp_z(arg1: mpq_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpq_div(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_div_2exp(arg1: mpq_ptr,
                           arg2: mpq_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpq_equal(arg1: mpq_srcptr, arg2: mpq_srcptr) -> c_int;
    pub fn __gmpq_get_num(arg1: mpz_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_get_den(arg1: mpz_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_get_d(arg1: mpq_srcptr) -> f64;
    pub fn __gmpq_get_str(arg1: *mut c_char,
                          arg2: c_int,
                          arg3: mpq_srcptr)
                          -> *mut c_char;
    pub fn __gmpq_init(arg1: mpq_ptr);
    pub fn __gmpq_inits(arg1: mpq_ptr, ...);
    pub fn __gmpq_inv(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_mul(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_mul_2exp(arg1: mpq_ptr,
                           arg2: mpq_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpq_neg(__gmp_w: mpq_ptr, __gmp_u: mpq_srcptr);
    pub fn __gmpq_set(arg1: mpq_ptr, arg2: mpq_srcptr);
    pub fn __gmpq_set_d(arg1: mpq_ptr, arg2: f64);
    pub fn __gmpq_set_den(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_set_f(arg1: mpq_ptr, arg2: mpf_srcptr);
    pub fn __gmpq_set_num(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_set_si(arg1: mpq_ptr, arg2: c_long, arg3: c_ulong);
    pub fn __gmpq_set_str(arg1: mpq_ptr,
                          arg2: *const c_char,
                          arg3: c_int)
                          -> c_int;
    pub fn __gmpq_set_ui(arg1: mpq_ptr, arg2: c_ulong, arg3: c_ulong);
    pub fn __gmpq_set_z(arg1: mpq_ptr, arg2: mpz_srcptr);
    pub fn __gmpq_sub(arg1: mpq_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr);
    pub fn __gmpq_swap(arg1: mpq_ptr, arg2: mpq_ptr);

    // Float (i.e. F) routines.

    pub fn __gmpf_abs(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_add(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_add_ui(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: c_ulong);
    pub fn __gmpf_ceil(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_clear(arg1: mpf_ptr);
    pub fn __gmpf_clears(arg1: mpf_ptr, ...);
    pub fn __gmpf_cmp(arg1: mpf_srcptr, arg2: mpf_srcptr) -> c_int;
    pub fn __gmpf_cmp_z(arg1: mpf_srcptr, arg2: mpz_srcptr) -> c_int;
    pub fn __gmpf_cmp_d(arg1: mpf_srcptr, arg2: f64) -> c_int;
    pub fn __gmpf_cmp_si(arg1: mpf_srcptr, arg2: c_long) -> c_int;
    pub fn __gmpf_cmp_ui(arg1: mpf_srcptr, arg2: c_ulong) -> c_int;
    pub fn __gmpf_div(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_div_2exp(arg1: mpf_ptr,
                           arg2: mpf_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpf_div_ui(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: c_ulong);
    pub fn __gmpf_dump(arg1: mpf_srcptr);
    pub fn __gmpf_eq(arg1: mpf_srcptr,
                     arg2: mpf_srcptr,
                     arg3: mp_bitcnt_t)
                     -> c_int;
    pub fn __gmpf_fits_sint_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_fits_slong_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_fits_sshort_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_fits_uint_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_fits_ulong_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_fits_ushort_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_floor(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_get_d(arg1: mpf_srcptr) -> f64;
    pub fn __gmpf_get_d_2exp(arg1: *mut c_long, arg2: mpf_srcptr) -> f64;
    pub fn __gmpf_get_default_prec() -> mp_bitcnt_t;
    pub fn __gmpf_get_prec(arg1: mpf_srcptr) -> mp_bitcnt_t;
    pub fn __gmpf_get_si(arg1: mpf_srcptr) -> c_long;
    pub fn __gmpf_get_str(arg1: *mut c_char,
                          arg2: *mut mp_exp_t,
                          arg3: c_int,
                          arg4: usize,
                          arg5: mpf_srcptr)
                          -> *mut c_char;
    pub fn __gmpf_get_ui(arg1: mpf_srcptr) -> c_ulong;
    pub fn __gmpf_init(arg1: mpf_ptr);
    pub fn __gmpf_init2(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_inits(arg1: mpf_ptr, ...);
    pub fn __gmpf_init_set(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_init_set_d(arg1: mpf_ptr, arg2: f64);
    pub fn __gmpf_init_set_si(arg1: mpf_ptr, arg2: c_long);
    pub fn __gmpf_init_set_str(arg1: mpf_ptr,
                               arg2: *const c_char,
                               arg3: c_int)
                               -> c_int;
    pub fn __gmpf_init_set_ui(arg1: mpf_ptr, arg2: c_ulong);
    pub fn __gmpf_integer_p(arg1: mpf_srcptr) -> c_int;
    pub fn __gmpf_mul(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_mul_2exp(arg1: mpf_ptr,
                           arg2: mpf_srcptr,
                           arg3: mp_bitcnt_t);
    pub fn __gmpf_mul_ui(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: c_ulong);
    pub fn __gmpf_neg(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_pow_ui(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: c_ulong);
    pub fn __gmpf_random2(arg1: mpf_ptr, arg2: mp_size_t, arg3: mp_exp_t);
    pub fn __gmpf_reldiff(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_set(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_set_d(arg1: mpf_ptr, arg2: f64);
    pub fn __gmpf_set_default_prec(arg1: mp_bitcnt_t);
    pub fn __gmpf_set_prec(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_set_prec_raw(arg1: mpf_ptr, arg2: mp_bitcnt_t);
    pub fn __gmpf_set_q(arg1: mpf_ptr, arg2: mpq_srcptr);
    pub fn __gmpf_set_si(arg1: mpf_ptr, arg2: c_long);
    pub fn __gmpf_set_str(arg1: mpf_ptr,
                          arg2: *const c_char,
                          arg3: c_int)
                          -> c_int;
    pub fn __gmpf_set_ui(arg1: mpf_ptr, arg2: c_ulong);
    pub fn __gmpf_set_z(arg1: mpf_ptr, arg2: mpz_srcptr);
    pub fn __gmpf_size(arg1: mpf_srcptr) -> usize;
    pub fn __gmpf_sqrt(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_sqrt_ui(arg1: mpf_ptr, arg2: c_ulong);
    pub fn __gmpf_sub(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr);
    pub fn __gmpf_sub_ui(arg1: mpf_ptr, arg2: mpf_srcptr, arg3: c_ulong);
    pub fn __gmpf_swap(arg1: mpf_ptr, arg2: mpf_ptr);
    pub fn __gmpf_trunc(arg1: mpf_ptr, arg2: mpf_srcptr);
    pub fn __gmpf_ui_div(arg1: mpf_ptr, arg2: c_ulong, arg3: mpf_srcptr);
    pub fn __gmpf_ui_sub(arg1: mpf_ptr, arg2: c_ulong, arg3: mpf_srcptr);
    pub fn __gmpf_urandomb(arg1: mpf_t,
                           arg2: gmp_randstate_t,
                           arg3: mp_bitcnt_t);

    // Low level positive-integer (i.e. N) routines.

    pub fn __gmpn_add(__gmp_wp: mp_ptr,
                      __gmp_xp: mp_srcptr,
                      __gmp_xsize: mp_size_t,
                      __gmp_yp: mp_srcptr,
                      __gmp_ysize: mp_size_t)
                      -> mp_limb_t;
    pub fn __gmpn_add_1(__gmp_dst: mp_ptr,
                        __gmp_src: mp_srcptr,
                        __gmp_size: mp_size_t,
                        __gmp_n: mp_limb_t)
                        -> mp_limb_t;
    pub fn __gmpn_add_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t)
                        -> mp_limb_t;
    pub fn __gmpn_addmul_1(arg1: mp_ptr,
                           arg2: mp_srcptr,
                           arg3: mp_size_t,
                           arg4: mp_limb_t)
                           -> mp_limb_t;
    pub fn __gmpn_cmp(__gmp_xp: mp_srcptr,
                      __gmp_yp: mp_srcptr,
                      __gmp_size: mp_size_t)
                      -> c_int;
    pub fn __gmpn_zero_p(__gmp_p: mp_srcptr, __gmp_n: mp_size_t) -> c_int;
    pub fn __gmpn_divexact_1(arg1: mp_ptr,
                             arg2: mp_srcptr,
                             arg3: mp_size_t,
                             arg4: mp_limb_t);
    pub fn __gmpn_divexact_by3c(arg1: mp_ptr,
                                arg2: mp_srcptr,
                                arg3: mp_size_t,
                                arg4: mp_limb_t)
                                -> mp_limb_t;
    pub fn __gmpn_divrem(arg1: mp_ptr,
                         arg2: mp_size_t,
                         arg3: mp_ptr,
                         arg4: mp_size_t,
                         arg5: mp_srcptr,
                         arg6: mp_size_t)
                         -> mp_limb_t;
    pub fn __gmpn_divrem_1(arg1: mp_ptr,
                           arg2: mp_size_t,
                           arg3: mp_srcptr,
                           arg4: mp_size_t,
                           arg5: mp_limb_t)
                           -> mp_limb_t;
    pub fn __gmpn_divrem_2(arg1: mp_ptr,
                           arg2: mp_size_t,
                           arg3: mp_ptr,
                           arg4: mp_size_t,
                           arg5: mp_srcptr)
                           -> mp_limb_t;
    pub fn __gmpn_div_qr_1(arg1: mp_ptr,
                           arg2: *mut mp_limb_t,
                           arg3: mp_srcptr,
                           arg4: mp_size_t,
                           arg5: mp_limb_t)
                           -> mp_limb_t;
    pub fn __gmpn_div_qr_2(arg1: mp_ptr,
                           arg2: mp_ptr,
                           arg3: mp_srcptr,
                           arg4: mp_size_t,
                           arg5: mp_srcptr)
                           -> mp_limb_t;
    pub fn __gmpn_gcd(arg1: mp_ptr,
                      arg2: mp_ptr,
                      arg3: mp_size_t,
                      arg4: mp_ptr,
                      arg5: mp_size_t)
                      -> mp_size_t;
    pub fn __gmpn_gcd_1(arg1: mp_srcptr,
                        arg2: mp_size_t,
                        arg3: mp_limb_t)
                        -> mp_limb_t;
    pub fn __gmpn_gcdext_1(arg1: *mut mp_limb_signed_t,
                           arg2: *mut mp_limb_signed_t,
                           arg3: mp_limb_t,
                           arg4: mp_limb_t)
                           -> mp_limb_t;
    pub fn __gmpn_gcdext(arg1: mp_ptr,
                         arg2: mp_ptr,
                         arg3: *mut mp_size_t,
                         arg4: mp_ptr,
                         arg5: mp_size_t,
                         arg6: mp_ptr,
                         arg7: mp_size_t)
                         -> mp_size_t;
    pub fn __gmpn_get_str(arg1: *mut c_uchar,
                          arg2: c_int,
                          arg3: mp_ptr,
                          arg4: mp_size_t)
                          -> usize;
    pub fn __gmpn_hamdist(arg1: mp_srcptr,
                          arg2: mp_srcptr,
                          arg3: mp_size_t)
                          -> mp_bitcnt_t;
    pub fn __gmpn_lshift(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_size_t,
                         arg4: c_uint)
                         -> mp_limb_t;
    pub fn __gmpn_mod_1(arg1: mp_srcptr,
                        arg2: mp_size_t,
                        arg3: mp_limb_t)
                        -> mp_limb_t;
    pub fn __gmpn_mul(arg1: mp_ptr,
                      arg2: mp_srcptr,
                      arg3: mp_size_t,
                      arg4: mp_srcptr,
                      arg5: mp_size_t)
                      -> mp_limb_t;
    pub fn __gmpn_mul_1(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_size_t,
                        arg4: mp_limb_t)
                        -> mp_limb_t;
    pub fn __gmpn_mul_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_sqr(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_neg(__gmp_rp: mp_ptr,
                      __gmp_up: mp_srcptr,
                      __gmp_n: mp_size_t)
                      -> mp_limb_t;
    pub fn __gmpn_com(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_perfect_square_p(arg1: mp_srcptr, arg2: mp_size_t) -> c_int;
    pub fn __gmpn_perfect_power_p(arg1: mp_srcptr, arg2: mp_size_t) -> c_int;
    pub fn __gmpn_popcount(arg1: mp_srcptr, arg2: mp_size_t) -> mp_bitcnt_t;
    pub fn __gmpn_pow_1(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_size_t,
                        arg4: mp_limb_t,
                        arg5: mp_ptr)
                        -> mp_size_t;
    pub fn __gmpn_preinv_mod_1(arg1: mp_srcptr,
                               arg2: mp_size_t,
                               arg3: mp_limb_t,
                               arg4: mp_limb_t)
                               -> mp_limb_t;
    pub fn __gmpn_random(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_random2(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_rshift(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_size_t,
                         arg4: c_uint)
                         -> mp_limb_t;
    pub fn __gmpn_scan0(arg1: mp_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpn_scan1(arg1: mp_srcptr, arg2: mp_bitcnt_t) -> mp_bitcnt_t;
    pub fn __gmpn_set_str(arg1: mp_ptr,
                          arg2: *const c_uchar,
                          arg3: usize,
                          arg4: c_int)
                          -> mp_size_t;
    pub fn __gmpn_sizeinbase(arg1: mp_srcptr,
                             arg2: mp_size_t,
                             arg3: c_int)
                             -> usize;
    pub fn __gmpn_sqrtrem(arg1: mp_ptr,
                          arg2: mp_ptr,
                          arg3: mp_srcptr,
                          arg4: mp_size_t)
                          -> mp_size_t;
    pub fn __gmpn_sub(__gmp_wp: mp_ptr,
                      __gmp_xp: mp_srcptr,
                      __gmp_xsize: mp_size_t,
                      __gmp_yp: mp_srcptr,
                      __gmp_ysize: mp_size_t)
                      -> mp_limb_t;
    pub fn __gmpn_sub_1(__gmp_dst: mp_ptr,
                        __gmp_src: mp_srcptr,
                        __gmp_size: mp_size_t,
                        __gmp_n: mp_limb_t)
                        -> mp_limb_t;
    pub fn __gmpn_sub_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t)
                        -> mp_limb_t;
    pub fn __gmpn_submul_1(arg1: mp_ptr,
                           arg2: mp_srcptr,
                           arg3: mp_size_t,
                           arg4: mp_limb_t)
                           -> mp_limb_t;
    pub fn __gmpn_tdiv_qr(arg1: mp_ptr,
                          arg2: mp_ptr,
                          arg3: mp_size_t,
                          arg4: mp_srcptr,
                          arg5: mp_size_t,
                          arg6: mp_srcptr,
                          arg7: mp_size_t);
    pub fn __gmpn_and_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_andn_n(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_nand_n(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_ior_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_iorn_n(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_nior_n(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_xor_n(arg1: mp_ptr,
                        arg2: mp_srcptr,
                        arg3: mp_srcptr,
                        arg4: mp_size_t);
    pub fn __gmpn_xnor_n(arg1: mp_ptr,
                         arg2: mp_srcptr,
                         arg3: mp_srcptr,
                         arg4: mp_size_t);
    pub fn __gmpn_copyi(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_copyd(arg1: mp_ptr, arg2: mp_srcptr, arg3: mp_size_t);
    pub fn __gmpn_zero(arg1: mp_ptr, arg2: mp_size_t);
    pub fn __gmpn_cnd_add_n(arg1: mp_limb_t,
                            arg2: mp_ptr,
                            arg3: mp_srcptr,
                            arg4: mp_srcptr,
                            arg5: mp_size_t)
                            -> mp_limb_t;
    pub fn __gmpn_cnd_sub_n(arg1: mp_limb_t,
                            arg2: mp_ptr,
                            arg3: mp_srcptr,
                            arg4: mp_srcptr,
                            arg5: mp_size_t)
                            -> mp_limb_t;
    pub fn __gmpn_sec_add_1(arg1: mp_ptr,
                            arg2: mp_srcptr,
                            arg3: mp_size_t,
                            arg4: mp_limb_t,
                            arg5: mp_ptr)
                            -> mp_limb_t;
    pub fn __gmpn_sec_add_1_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_sub_1(arg1: mp_ptr,
                            arg2: mp_srcptr,
                            arg3: mp_size_t,
                            arg4: mp_limb_t,
                            arg5: mp_ptr)
                            -> mp_limb_t;
    pub fn __gmpn_sec_sub_1_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_cnd_swap(arg1: mp_limb_t,
                           arg2: *mut mp_limb_t,
                           arg3: *mut mp_limb_t,
                           arg4: mp_size_t);
    pub fn __gmpn_sec_mul(arg1: mp_ptr,
                          arg2: mp_srcptr,
                          arg3: mp_size_t,
                          arg4: mp_srcptr,
                          arg5: mp_size_t,
                          arg6: mp_ptr);
    pub fn __gmpn_sec_mul_itch(arg1: mp_size_t, arg2: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_sqr(arg1: mp_ptr,
                          arg2: mp_srcptr,
                          arg3: mp_size_t,
                          arg4: mp_ptr);
    pub fn __gmpn_sec_sqr_itch(arg1: mp_size_t) -> mp_size_t;
    pub fn __gmpn_sec_powm(arg1: mp_ptr,
                           arg2: mp_srcptr,
                           arg3: mp_size_t,
                           arg4: mp_srcptr,
                           arg5: mp_bitcnt_t,
                           arg6: mp_srcptr,
                           arg7: mp_size_t,
                           arg8: mp_ptr);
    pub fn __gmpn_sec_powm_itch(arg1: mp_size_t,
                                arg2: mp_bitcnt_t,
                                arg3: mp_size_t)
                                -> mp_size_t;
    pub fn __gmpn_sec_tabselect(arg1: *mut mp_limb_t,
                                arg2: *const mp_limb_t,
                                arg3: mp_size_t,
                                arg4: mp_size_t,
                                arg5: mp_size_t);
    pub fn __gmpn_sec_div_qr(arg1: mp_ptr,
                             arg2: mp_ptr,
                             arg3: mp_size_t,
                             arg4: mp_srcptr,
                             arg5: mp_size_t,
                             arg6: mp_ptr)
                             -> mp_limb_t;
    pub fn __gmpn_sec_div_qr_itch(arg1: mp_size_t,
                                  arg2: mp_size_t)
                                  -> mp_size_t;
    pub fn __gmpn_sec_div_r(arg1: mp_ptr,
                            arg2: mp_size_t,
                            arg3: mp_srcptr,
                            arg4: mp_size_t,
                            arg5: mp_ptr);
    pub fn __gmpn_sec_div_r_itch(arg1: mp_size_t,
                                 arg2: mp_size_t)
                                 -> mp_size_t;
    pub fn __gmpn_sec_invert(arg1: mp_ptr,
                             arg2: mp_ptr,
                             arg3: mp_srcptr,
                             arg4: mp_size_t,
                             arg5: mp_bitcnt_t,
                             arg6: mp_ptr)
                             -> c_int;
    pub fn __gmpn_sec_invert_itch(arg1: mp_size_t) -> mp_size_t;
}

#[inline]
pub unsafe fn mpz_sgn(Z: mpz_srcptr) -> c_int {
    if (*Z)._mp_size < 0 {
        -1
    } else if (*Z)._mp_size > 0 {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn mpf_sgn(F: mpf_srcptr) -> c_int {
    if (*F)._mp_size < 0 {
        -1
    } else if (*F)._mp_size > 0 {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn mpq_sgn(Q: mpq_srcptr) -> c_int {
    if (*Q)._mp_num._mp_size < 0 {
        -1
    } else if (*Q)._mp_num._mp_size > 0 {
        1
    } else {
        0
    }
}

#[inline]
pub unsafe fn mpz_odd_p(z: mpz_srcptr) -> c_int {
    (*(*z)._mp_d) as c_int & if (*z)._mp_size != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn mpz_even_p(z: mpz_srcptr) -> c_int {
    !mpz_odd_p(z)
}
