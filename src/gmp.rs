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

macro_rules! c_static {
    { $($name:ident = $c:ident: $ty:ty;)* } => {
        $(
            #[link(name = "gmp", kind = "static")]
            extern "C" {
                pub static $c: $ty;
            }
            pub use self::$c as $name;
        )*
    };
}

macro_rules! c_fn {
    { $($name:ident = $c:ident
        $( ($($par:ident: $ty:ty),* $(; $dots:tt)*) $(-> $ret:ty)*
        )* ;
    )* } => {
        $(
            $(
                #[link(name = "gmp", kind = "static")]
                extern "C" {
                    pub fn $c($($par: $ty),* $(, $dots)*) $(-> $ret)*;
                }
            )*
            pub use self::$c as $name;
        )*
    };
}

// Useful constants

c_static! {
    mp_bits_per_limb = __gmp_bits_per_limb: c_int;
}
pub const __GNU_MP_VERSION: c_int = 6;
pub const __GNU_MP_VERSION_MINOR: c_int = 1;
pub const __GNU_MP_VERSION_PATCHLEVEL: c_int = 2;
c_static! {
    gmp_version = __gmp_version: *const c_char;
}

// Types

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = __mpz_struct;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpq_struct {
    pub _mp_num: __mpz_struct,
    pub _mp_den: __mpz_struct,
}
pub type mpq_t = __mpq_struct;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpf_struct {
    pub _mp_prec: c_int,
    pub _mp_size: c_int,
    pub _mp_exp: mp_exp_t,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpf_t = __mpf_struct;

pub type mp_exp_t = c_long;
pub type mp_limb_t = c_ulong;
pub type mp_size_t = c_long;
pub type mp_bitcnt_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
enum gmp_randalg_t {
    _GMP_RAND_ALG_DEFAULT = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __gmp_randstate_struct {
    _mp_seed: [__mpz_struct; 1],
    _mp_alg: gmp_randalg_t,
    _mp_algdata: __gmp_randstate_struct_union,
}
pub type gmp_randstate_t = __gmp_randstate_struct;

// this union has only one option, so just use a struct
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct __gmp_randstate_struct_union {
    _mp_lc: *mut c_void,
}

// Types for function declarations in this file.

type mpz_srcptr = *const __mpz_struct;
type mpz_ptr = *mut __mpz_struct;
type mpq_srcptr = *const __mpq_struct;
type mpq_ptr = *mut __mpq_struct;
type mpf_srcptr = *const __mpf_struct;
type mpf_ptr = *mut __mpf_struct;
type mp_ptr = *mut mp_limb_t;
type mp_srcptr = *const mp_limb_t;
type randstate_srcptr = *const __gmp_randstate_struct;
type randstate_ptr = *mut __gmp_randstate_struct;

// Integers

c_fn! {
    // Initialization Functions
    mpz_init = __gmpz_init(x: mpz_ptr);
    mpz_inits = __gmpz_inits(x: mpz_ptr; ...);
    mpz_init2 = __gmpz_init2(x: mpz_ptr, n: mp_bitcnt_t);
    mpz_clear = __gmpz_clear(x: mpz_ptr);
    mpz_clears = __gmpz_clears(x: mpz_ptr; ...);
    mpz_realloc2 = __gmpz_realloc2(x: mpz_ptr, n: mp_bitcnt_t);

    // Assignment Functions
    mpz_set = __gmpz_set(rop: mpz_ptr, op: mpz_srcptr);
    mpz_set_ui = __gmpz_set_ui(rop: mpz_ptr, op: c_ulong);
    mpz_set_si = __gmpz_set_si(rop: mpz_ptr, op: c_long);
    mpz_set_d = __gmpz_set_d(rop: mpz_ptr, op: f64);
    mpz_set_q = __gmpz_set_q(rop: mpz_ptr, op: mpq_srcptr);
    mpz_set_f = __gmpz_set_f(rop: mpz_ptr, op: mpf_srcptr);
    mpz_set_str = __gmpz_set_str(rop: mpz_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    mpz_swap = __gmpz_swap(rop1: mpz_ptr, rop2: mpz_ptr);

    // Combined Initialization and Assignment Functions
    mpz_init_set = __gmpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    mpz_init_set_ui = __gmpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    mpz_init_set_si = __gmpz_init_set_si(rop: mpz_ptr, op: c_long);
    mpz_init_set_d = __gmpz_init_set_d(rop: mpz_ptr, op: f64);
    mpz_init_set_str = __gmpz_init_set_str(rop: mpz_ptr,
                                           str: *const c_char,
                                           base: c_int)
                                           -> c_int;

    // Conversion Functions
    mpz_get_ui = __gmpz_get_ui(op: mpz_srcptr) -> c_ulong;
    mpz_get_si = __gmpz_get_si(op: mpz_srcptr) -> c_long;
    mpz_get_d = __gmpz_get_d(op: mpz_srcptr) -> f64;
    mpz_get_d_2exp = __gmpz_get_d_2exp(exp: *mut c_long, op: mpz_srcptr) -> f64;
    mpz_get_str = __gmpz_get_str(str: *mut c_char,
                                 base: c_int,
                                 op: mpz_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    mpz_add = __gmpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_add_ui = __gmpz_add_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    mpz_sub = __gmpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_sub_ui = __gmpz_sub_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    mpz_ui_sub = __gmpz_ui_sub(rop: mpz_ptr, op1: c_ulong, op2: mpz_srcptr);
    mpz_mul = __gmpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_mul_si = __gmpz_mul_si(rop: mpz_ptr, op1: mpz_srcptr, op2: c_long);
    mpz_mul_ui = __gmpz_mul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    mpz_addmul = __gmpz_addmul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_addmul_ui = __gmpz_addmul_ui(rop: mpz_ptr,
                                     op1: mpz_srcptr,
                                     op2: c_ulong);
    mpz_submul = __gmpz_submul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_submul_ui = __gmpz_submul_ui(rop: mpz_ptr,
                                     op1: mpz_srcptr,
                                     op2: c_ulong);
    mpz_mul_2exp = __gmpz_mul_2exp(rop: mpz_ptr,
                                   op1: mpz_srcptr,
                                   op2: mp_bitcnt_t);
    mpz_neg = __gmpz_neg(rop: mpz_ptr, op: mpz_srcptr);
    mpz_abs = __gmpz_abs(rop: mpz_ptr, op: mpz_srcptr);

    // Division Functions
    mpz_cdiv_q = __gmpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_cdiv_r = __gmpz_cdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_cdiv_qr = __gmpz_cdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    mpz_cdiv_q_ui = __gmpz_cdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong)
                                     -> c_ulong;
    mpz_cdiv_r_ui = __gmpz_cdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    mpz_cdiv_qr_ui = __gmpz_cdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    mpz_cdiv_ui = __gmpz_cdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    mpz_cdiv_q_2exp = __gmpz_cdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_cdiv_r_2exp = __gmpz_cdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_fdiv_q = __gmpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_fdiv_r = __gmpz_fdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_fdiv_qr = __gmpz_fdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    mpz_fdiv_q_ui = __gmpz_fdiv_q_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    mpz_fdiv_r_ui = __gmpz_fdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    mpz_fdiv_qr_ui = __gmpz_fdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    mpz_fdiv_ui = __gmpz_fdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    mpz_fdiv_q_2exp = __gmpz_fdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_fdiv_r_2exp = __gmpz_fdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_tdiv_q = __gmpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_tdiv_r = __gmpz_tdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_tdiv_qr = __gmpz_tdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    mpz_tdiv_q_ui = __gmpz_tdiv_q_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    mpz_tdiv_r_ui = __gmpz_tdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    mpz_tdiv_qr_ui = __gmpz_tdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    mpz_tdiv_ui = __gmpz_tdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    mpz_tdiv_q_2exp = __gmpz_tdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_tdiv_r_2exp = __gmpz_tdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: mp_bitcnt_t);
    mpz_mod = __gmpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_mod_ui = __gmpz_fdiv_r_ui;
    mpz_divexact = __gmpz_divexact(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    mpz_divexact_ui = __gmpz_divexact_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong);
    mpz_divisible_p = __gmpz_divisible_p(n: mpz_srcptr, d: mpz_srcptr) -> c_int;
    mpz_divisible_ui_p = __gmpz_divisible_ui_p(n: mpz_srcptr,
                                               d: c_ulong)
                                               -> c_int;
    mpz_divisible_2exp_p = __gmpz_divisible_2exp_p(n: mpz_srcptr,
                                                   b: mp_bitcnt_t)
                                                   -> c_int;
    mpz_congruent_p = __gmpz_congruent_p(n: mpz_srcptr,
                                         c: mpz_srcptr,
                                         d: mpz_srcptr)
                                         -> c_int;
    mpz_congruent_ui_p = __gmpz_congruent_ui_p(n: mpz_srcptr,
                                               c: c_ulong,
                                               d: c_ulong)
                                               -> c_int;
    mpz_congruent_2exp_p = __gmpz_congruent_2exp_p(n: mpz_srcptr,
                                                   c: mpz_srcptr,
                                                   b: mp_bitcnt_t)
                                                   -> c_int;

    // Exponentiation Functions
    mpz_powm = __gmpz_powm(rop: mpz_ptr,
                           base: mpz_srcptr,
                           exp: mpz_srcptr,
                           modu: mpz_srcptr);
    mpz_powm_ui = __gmpz_powm_ui(rop: mpz_ptr,
                                 base: mpz_srcptr,
                                 exp: c_ulong,
                                 modu: mpz_srcptr);
    mpz_powm_sec = __gmpz_powm_sec(rop: mpz_ptr,
                                   base: mpz_srcptr,
                                   exp: mpz_srcptr,
                                   modu: mpz_srcptr);
    mpz_pow_ui = __gmpz_pow_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong);
    mpz_ui_pow_ui = __gmpz_ui_pow_ui(rop: mpz_ptr, base: c_ulong, exp: c_ulong);

    // Root Extraction Functions
    mpz_root = __gmpz_root(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong) -> c_int;
    mpz_rootrem = __gmpz_rootrem(root: mpz_ptr,
                                 rem: mpz_ptr,
                                 op: mpz_srcptr,
                                 n: c_ulong);
    mpz_sqrt = __gmpz_sqrt(rop: mpz_ptr, op: mpz_srcptr);
    mpz_sqrtrem = __gmpz_sqrtrem(rop1: mpz_ptr, rop2: mpz_ptr, op: mpz_srcptr);
    mpz_perfect_power_p = __gmpz_perfect_power_p(op: mpz_srcptr) -> c_int;
    mpz_perfect_square_p = __gmpz_perfect_square_p(op: mpz_srcptr) -> c_int;

    // Number Theoretic Functions
    mpz_probab_prime_p = __gmpz_probab_prime_p(n: mpz_srcptr,
                                               reps: c_int)
                                               -> c_int;
    mpz_nextprime = __gmpz_nextprime(rop: mpz_ptr, op: mpz_srcptr);
    mpz_gcd = __gmpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_gcd_ui = __gmpz_gcd_ui(rop: mpz_ptr,
                               op1: mpz_srcptr,
                               op2: c_ulong)
                               -> c_ulong;
    mpz_gcdext = __gmpz_gcdext(g: mpz_ptr,
                               s: mpz_ptr,
                               t: mpz_ptr,
                               a: mpz_srcptr,
                               b: mpz_srcptr);
    mpz_lcm = __gmpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_lcm_ui = __gmpz_lcm_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    mpz_invert = __gmpz_invert(rop: mpz_ptr,
                               op1: mpz_srcptr,
                               op2: mpz_srcptr)
                               -> c_int;
    mpz_jacobi = __gmpz_jacobi(a: mpz_srcptr, b: mpz_srcptr) -> c_int;
    mpz_legendre = __gmpz_jacobi;
    mpz_kronecker = __gmpz_jacobi;
    mpz_kronecker_si = __gmpz_kronecker_si(a: mpz_srcptr, b: c_long) -> c_int;
    mpz_kronecker_ui = __gmpz_kronecker_ui(a: mpz_srcptr, b: c_ulong) -> c_int;
    mpz_si_kronecker = __gmpz_si_kronecker(a: c_long, b: mpz_srcptr) -> c_int;
    mpz_ui_kronecker = __gmpz_ui_kronecker(a: c_ulong, b: mpz_srcptr) -> c_int;
    mpz_remove = __gmpz_remove(rop: mpz_ptr,
                               op: mpz_srcptr,
                               f: mpz_srcptr)
                               -> mp_bitcnt_t;
    mpz_fac_ui = __gmpz_fac_ui(rop: mpz_ptr, n: c_ulong);
    mpz_2fac_ui = __gmpz_2fac_ui(rop: mpz_ptr, n: c_ulong);
    mpz_mfac_uiui = __gmpz_mfac_uiui(rop: mpz_ptr, n: c_ulong, m: c_ulong);
    mpz_primorial_ui = __gmpz_primorial_ui(r: mpz_ptr, n: c_ulong);
    mpz_bin_ui = __gmpz_bin_ui(rop: mpz_ptr, n: mpz_srcptr, k: c_ulong);
    mpz_bin_uiui = __gmpz_bin_uiui(rop: mpz_ptr, n: c_ulong, k: c_ulong);
    mpz_fib_ui = __gmpz_fib_ui(f_n: mpz_ptr, n: c_ulong);
    mpz_fib2_ui = __gmpz_fib2_ui(f_n: mpz_ptr, fnsub1: mpz_ptr, n: c_ulong);
    mpz_lucnum_ui = __gmpz_lucnum_ui(ln: mpz_ptr, n: c_ulong);
    mpz_lucnum2_ui = __gmpz_lucnum2_ui(ln: mpz_ptr,
                                       lnsub1: mpz_ptr,
                                       n: c_ulong);

    // Comparison Functions
    mpz_cmp = __gmpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    mpz_cmp_d = __gmpz_cmp_d(op1: mpz_srcptr, op2: f64) -> c_int;
    mpz_cmp_si = __gmpz_cmp_si(op1: mpz_srcptr, op2: c_long) -> c_int;
    mpz_cmp_ui = __gmpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    mpz_cmpabs = __gmpz_cmpabs(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    mpz_cmpabs_d = __gmpz_cmpabs_d(op1: mpz_srcptr, op2: f64) -> c_int;
    mpz_cmpabs_ui = __gmpz_cmpabs_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
}
#[inline]
pub unsafe fn mpz_sgn(op: mpz_srcptr) -> c_int {
    if (*op)._mp_size < 0 {
        -1
    } else if (*op)._mp_size > 0 {
        1
    } else {
        0
    }
}
c_fn! {
    mpz_and = __gmpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_ior = __gmpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_xor = __gmpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    mpz_com = __gmpz_com(rop: mpz_ptr, op: mpz_srcptr);
    mpz_popcount = __gmpz_popcount(op: mpz_srcptr) -> mp_bitcnt_t;
    mpz_hamdist = __gmpz_hamdist(op1: mpz_srcptr,
                                 op2: mpz_srcptr)
                                 -> mp_bitcnt_t;
    mpz_scan0 = __gmpz_scan0(op: mpz_srcptr,
                             starting_bit: mp_bitcnt_t)
                             -> mp_bitcnt_t;
    mpz_scan1 = __gmpz_scan1(op: mpz_srcptr,
                             starting_bit: mp_bitcnt_t)
                             -> mp_bitcnt_t;
    mpz_setbit = __gmpz_setbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    mpz_clrbit = __gmpz_clrbit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    mpz_combit = __gmpz_combit(rop: mpz_ptr, bit_index: mp_bitcnt_t);
    mpz_tstbit = __gmpz_tstbit(rop: mpz_srcptr,
                               bit_index: mp_bitcnt_t)
                               -> c_int;

    // Random Number Functions
    mpz_urandomb = __gmpz_urandomb(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: mp_bitcnt_t);
    mpz_urandomm = __gmpz_urandomm(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: mpz_srcptr);
    mpz_rrandomb = __gmpz_rrandomb(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: mp_bitcnt_t);
    mpz_random = __gmpz_random(rop: mpz_ptr, max_size: mp_size_t);
    mpz_random2 = __gmpz_random2(rop: mpz_ptr, max_size: mp_size_t);

    // Integer Import and Export
    mpz_import = __gmpz_import(rop: mpz_ptr,
                               count: usize,
                               order: c_int,
                               size: usize,
                               endian: c_int,
                               nails: usize,
                               op: *const c_void);
    mpz_export = __gmpz_export(rop: *mut c_void,
                               countp: *mut usize,
                               order: c_int,
                               size: usize,
                               endian: c_int,
                               nails: usize,
                               op: mpz_srcptr)
                               -> *mut c_void;

    // Miscellaneous Functions
    mpz_fits_ulong_p = __gmpz_fits_ulong_p(op: mpz_srcptr) -> c_int;
    mpz_fits_slong_p = __gmpz_fits_slong_p(op: mpz_srcptr) -> c_int;
    mpz_fits_uint_p = __gmpz_fits_uint_p(op: mpz_srcptr) -> c_int;
    mpz_fits_sint_p = __gmpz_fits_sint_p(op: mpz_srcptr) -> c_int;
    mpz_fits_ushort_p = __gmpz_fits_ushort_p(op: mpz_srcptr) -> c_int;
    mpz_fits_sshort_p = __gmpz_fits_sshort_p(op: mpz_srcptr) -> c_int;
}
#[inline]
pub unsafe fn mpz_odd_p(op: mpz_srcptr) -> c_int {
    (*(*op)._mp_d) as c_int & if (*op)._mp_size != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn mpz_even_p(op: mpz_srcptr) -> c_int {
    !mpz_odd_p(op)
}
c_fn! {
    mpz_sizeinbase = __gmpz_sizeinbase(arg1: mpz_srcptr, arg2: c_int) -> usize;

    // Special Functions
    mpz_array_init = __gmpz_array_init(integer_array: mpz_ptr,
                                       array_size: mp_size_t,
                                       fixed_num_bits: mp_size_t);
    _mpz_realloc = __gmpz_realloc(integer: mpz_ptr,
                                  new_alloc: mp_size_t)
                                  -> *mut c_void;
    mpz_getlimbn = __gmpz_getlimbn(op: mpz_srcptr, n: mp_size_t) -> mp_limb_t;
    mpz_size = __gmpz_size(op: mpz_srcptr) -> usize;
    mpz_limbs_read = __gmpz_limbs_read(x: mpz_srcptr) -> mp_srcptr;
    mpz_limbs_write = __gmpz_limbs_write(x: mpz_ptr, n: mp_size_t) -> mp_ptr;
    mpz_limbs_modify = __gmpz_limbs_modify(x: mpz_ptr, n: mp_size_t) -> mp_ptr;
    mpz_limbs_finish = __gmpz_limbs_finish(x: mpz_ptr, s: mp_size_t);
    mpz_roinit_n = __gmpz_roinit_n(x: mpz_ptr,
                                   xp: mp_srcptr,
                                   xs: mp_size_t)
                                   -> mpz_srcptr;
}
#[inline]
pub unsafe fn MPZ_ROINIT_N(xp: mp_ptr, xs: mp_size_t) -> mpz_t {
    __mpz_struct {
        _mp_alloc: 0,
        _mp_size: xs as c_int,
        _mp_d: xp,
    }
}

// Rational numbers

c_fn! {
    mpq_canonicalize = __gmpq_canonicalize(op: mpq_ptr);

    // Initialization and Assignment Functions
    mpq_init = __gmpq_init(x: mpq_ptr);
    mpq_inits = __gmpq_inits(x: mpq_ptr; ...);
    mpq_clear = __gmpq_clear(x: mpq_ptr);
    mpq_clears = __gmpq_clears(x: mpq_ptr; ...);
    mpq_set = __gmpq_set(rop: mpq_ptr, op: mpq_srcptr);
    mpq_set_z = __gmpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    mpq_set_ui = __gmpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    mpq_set_si = __gmpq_set_si(rop: mpq_ptr, op1: c_long, op2: c_ulong);
    mpq_set_str = __gmpq_set_str(rop: mpq_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    mpq_swap = __gmpq_swap(rop1: mpq_ptr, rop2: mpq_ptr);

    // Conversion Functions
    mpq_get_d = __gmpq_get_d(op: mpq_srcptr) -> f64;
    mpq_set_d = __gmpq_set_d(rop: mpq_ptr, op: f64);
    mpq_set_f = __gmpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    mpq_get_str = __gmpq_get_str(str: *mut c_char,
                                 base: c_int,
                                 op: mpq_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    mpq_add = __gmpq_add(sum: mpq_ptr,
                         addend1: mpq_srcptr,
                         addend2: mpq_srcptr);
    mpq_sub = __gmpq_sub(difference: mpq_ptr,
                         minuend: mpq_srcptr,
                         subtrahend: mpq_srcptr);
    mpq_mul = __gmpq_mul(product: mpq_ptr,
                         multiplier: mpq_srcptr,
                         multiplicand: mpq_srcptr);
    mpq_mul_2exp = __gmpq_mul_2exp(rop: mpq_ptr,
                                   op1: mpq_srcptr,
                                   op2: mp_bitcnt_t);
    mpq_div = __gmpq_div(quotient: mpq_ptr,
                         dividend: mpq_srcptr,
                         divisor: mpq_srcptr);
    mpq_div_2exp = __gmpq_div_2exp(rop: mpq_ptr,
                                   op1: mpq_srcptr,
                                   op2: mp_bitcnt_t);
    mpq_neg = __gmpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr);
    mpq_abs = __gmpq_abs(rop: mpq_ptr, op: mpq_srcptr);
    mpq_inv = __gmpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);

    // Comparison Functions
    mpq_cmp = __gmpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    mpq_cmp_z = __gmpq_cmp_z(op1: mpq_srcptr, op2: mpz_srcptr) -> c_int;
    mpq_cmp_ui = __gmpq_cmp_ui(op1: mpq_srcptr,
                               num2: c_ulong,
                               den2: c_ulong)
                               -> c_int;
    mpq_cmp_si = __gmpq_cmp_si(op1: mpq_srcptr,
                               num2: c_long,
                               den2: c_ulong)
                               -> c_int;
}
#[inline]
pub unsafe fn mpq_sgn(op: mpq_srcptr) -> c_int {
    if (*op)._mp_num._mp_size < 0 {
        -1
    } else if (*op)._mp_num._mp_size > 0 {
        1
    } else {
        0
    }
}
c_fn! {
    mpq_equal = __gmpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;

    // Applying Integer Functions to Rationals
}
#[inline]
pub unsafe fn mpq_numref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op)._mp_num) as mpz_ptr
}
#[inline]
pub unsafe fn mpq_denref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op)._mp_den) as mpz_ptr
}
c_fn! {
    mpq_get_num = __gmpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    mpq_get_den = __gmpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    mpq_set_den = __gmpq_set_den(rational: mpq_ptr, numerator: mpz_srcptr);
    mpq_set_num = __gmpq_set_num(rational: mpq_ptr, denominator: mpz_srcptr);
}

// Floating-point numbers

c_fn! {
    // Initialization Functions
    mpf_set_default_prec = __gmpf_set_default_prec(prec: mp_bitcnt_t);
    mpf_get_default_prec = __gmpf_get_default_prec() -> mp_bitcnt_t;
    mpf_init = __gmpf_init(x: mpf_ptr);
    mpf_init2 = __gmpf_init2(x: mpf_ptr, prec: mp_bitcnt_t);
    mpf_inits = __gmpf_inits(x: mpf_ptr; ...);
    mpf_clear = __gmpf_clear(x: mpf_ptr);
    mpf_clears = __gmpf_clears(x: mpf_ptr; ...);
    mpf_get_prec = __gmpf_get_prec(op: mpf_srcptr) -> mp_bitcnt_t;
    mpf_set_prec = __gmpf_set_prec(rop: mpf_ptr, prec: mp_bitcnt_t);
    mpf_set_prec_raw = __gmpf_set_prec_raw(rop: mpf_ptr, prec: mp_bitcnt_t);

    // Assignment Functions
    mpf_set = __gmpf_set(rop: mpf_ptr, op: mpf_srcptr);
    mpf_set_ui = __gmpf_set_ui(rop: mpf_ptr, op: c_ulong);
    mpf_set_si = __gmpf_set_si(rop: mpf_ptr, op: c_long);
    mpf_set_d = __gmpf_set_d(rop: mpf_ptr, op: f64);
    mpf_set_z = __gmpf_set_z(rop: mpf_ptr, op: mpz_srcptr);
    mpf_set_q = __gmpf_set_q(rop: mpf_ptr, op: mpq_srcptr);
    mpf_set_str = __gmpf_set_str(rop: mpf_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    mpf_swap = __gmpf_swap(rop1: mpf_ptr, rop2: mpf_ptr);

    // Combined Initialization and Assignment Functions
    mpf_init_set = __gmpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    mpf_init_set_ui = __gmpf_init_set_ui(rop: mpf_ptr, op: c_ulong);
    mpf_init_set_si = __gmpf_init_set_si(rop: mpf_ptr, op: c_long);
    mpf_init_set_d = __gmpf_init_set_d(rop: mpf_ptr, op: f64);
    mpf_init_set_str = __gmpf_init_set_str(rop: mpf_ptr,
                                           str: *const c_char,
                                           base: c_int)
                                           -> c_int;

    // Conversion Functions
    mpf_get_d = __gmpf_get_d(op: mpf_srcptr) -> f64;
    mpf_get_d_2exp = __gmpf_get_d_2exp(exp: *mut c_long, op: mpf_srcptr) -> f64;
    mpf_get_si = __gmpf_get_si(op: mpf_srcptr) -> c_long;
    mpf_get_ui = __gmpf_get_ui(op: mpf_srcptr) -> c_ulong;
    mpf_get_str = __gmpf_get_str(str: *mut c_char,
                                 expptr: *mut mp_exp_t,
                                 base: c_int,
                                 n_digits: usize,
                                 op: mpf_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    mpf_add = __gmpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    mpf_add_ui = __gmpf_add_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    mpf_sub = __gmpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    mpf_ui_sub = __gmpf_ui_sub(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    mpf_sub_ui = __gmpf_sub_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    mpf_mul = __gmpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    mpf_mul_ui = __gmpf_mul_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    mpf_div = __gmpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    mpf_ui_div = __gmpf_ui_div(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    mpf_div_ui = __gmpf_div_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    mpf_sqrt = __gmpf_sqrt(rop: mpf_ptr, op: mpf_srcptr);
    mpf_sqrt_ui = __gmpf_sqrt_ui(rop: mpf_ptr, op: c_ulong);
    mpf_pow_ui = __gmpf_pow_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    mpf_neg = __gmpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    mpf_abs = __gmpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    mpf_mul_2exp = __gmpf_mul_2exp(rop: mpf_ptr,
                                   op1: mpf_srcptr,
                                   op2: mp_bitcnt_t);
    mpf_div_2exp = __gmpf_div_2exp(rop: mpf_ptr,
                                   op1: mpf_srcptr,
                                   op2: mp_bitcnt_t);

    // Comparison Functions
    mpf_cmp = __gmpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    mpf_cmp_z = __gmpf_cmp_z(op1: mpf_srcptr, op2: mpz_srcptr) -> c_int;
    mpf_cmp_d = __gmpf_cmp_d(op1: mpf_srcptr, op2: f64) -> c_int;
    mpf_cmp_ui = __gmpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    mpf_cmp_si = __gmpf_cmp_si(op1: mpf_srcptr, op2: c_long) -> c_int;
    mpf_eq = __gmpf_eq(op1: mpf_srcptr,
                       op2: mpf_srcptr,
                       op3: mp_bitcnt_t)
                       -> c_int;
    mpf_reldiff = __gmpf_reldiff(rop: mpf_ptr,
                                 op1: mpf_srcptr,
                                 op2: mpf_srcptr);
}
#[inline]
pub unsafe fn mpf_sgn(op: mpf_srcptr) -> c_int {
    if (*op)._mp_size < 0 {
        -1
    } else if (*op)._mp_size > 0 {
        1
    } else {
        0
    }
}
c_fn! {

    // Miscellaneous Functions
    mpf_ceil = __gmpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    mpf_floor = __gmpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    mpf_trunc = __gmpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
    mpf_integer_p = __gmpf_integer_p(op: mpf_srcptr) -> c_int;
    mpf_fits_ulong_p = __gmpf_fits_ulong_p(op: mpf_srcptr) -> c_int;
    mpf_fits_slong_p = __gmpf_fits_slong_p(op: mpf_srcptr) -> c_int;
    mpf_fits_uint_p = __gmpf_fits_uint_p(op: mpf_srcptr) -> c_int;
    mpf_fits_sint_p = __gmpf_fits_sint_p(op: mpf_srcptr) -> c_int;
    mpf_fits_ushort_p = __gmpf_fits_ushort_p(op: mpf_srcptr) -> c_int;
    mpf_fits_sshort_p = __gmpf_fits_sshort_p(op: mpf_srcptr) -> c_int;
    mpf_urandomb = __gmpf_urandomb(rop: mpf_t,
                                   state: randstate_ptr,
                                   nbits: mp_bitcnt_t);
    mpf_random2 = __gmpf_random2(rop: mpf_ptr,
                                 max_size: mp_size_t,
                                 exp: mp_exp_t);
}

// Low-Level Functions

c_fn! {
    mpn_add_n = __gmpn_add_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t)
                             -> mp_limb_t;
    mpn_add_1 = __gmpn_add_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: mp_size_t,
                             s2limb: mp_limb_t)
                             -> mp_limb_t;
    mpn_add = __gmpn_add(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: mp_size_t,
                         s2p: mp_srcptr,
                         s2n: mp_size_t)
                         -> mp_limb_t;
    mpn_sub_n = __gmpn_sub_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t)
                             -> mp_limb_t;
    mpn_sub_1 = __gmpn_sub_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: mp_size_t,
                             s2limb: mp_limb_t)
                             -> mp_limb_t;
    mpn_sub = __gmpn_sub(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: mp_size_t,
                         s2p: mp_srcptr,
                         s2n: mp_size_t)
                         -> mp_limb_t;
    mpn_neg = __gmpn_neg(rp: mp_ptr, sp: mp_srcptr, n: mp_size_t) -> mp_limb_t;
    mpn_mul_n = __gmpn_mul_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t);
    mpn_mul = __gmpn_mul(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: mp_size_t,
                         s2p: mp_srcptr,
                         s2n: mp_size_t)
                         -> mp_limb_t;
    mpn_sqr = __gmpn_sqr(rp: mp_ptr, s1p: mp_srcptr, n: mp_size_t);
    mpn_mul_1 = __gmpn_mul_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: mp_size_t,
                             s2limb: mp_limb_t)
                             -> mp_limb_t;
    mpn_addmul_1 = __gmpn_addmul_1(rp: mp_ptr,
                                   s1p: mp_srcptr,
                                   n: mp_size_t,
                                   s2limb: mp_limb_t)
                                   -> mp_limb_t;
    mpn_submul_1 = __gmpn_submul_1(rp: mp_ptr,
                                   s1p: mp_srcptr,
                                   n: mp_size_t,
                                   s2limb: mp_limb_t)
                                   -> mp_limb_t;
    mpn_tdiv_qr = __gmpn_tdiv_qr(qp: mp_ptr,
                                 rp: mp_ptr,
                                 qxn: mp_size_t,
                                 np: mp_srcptr,
                                 nn: mp_size_t,
                                 dp: mp_srcptr,
                                 dn: mp_size_t);
    mpn_divrem = __gmpn_divrem(r1p: mp_ptr,
                               sqn: mp_size_t,
                               rs2p: mp_ptr,
                               rs2n: mp_size_t,
                               s3p: mp_srcptr,
                               s3n: mp_size_t)
                               -> mp_limb_t;
    mpn_divrem_1 = __gmpn_divrem_1(r1p: mp_ptr,
                                   qxn: mp_size_t,
                                   s2p: mp_srcptr,
                                   s2n: mp_size_t,
                                   s3limb: mp_limb_t)
                                   -> mp_limb_t;
}
#[inline]
pub unsafe fn mpn_divmod_1(r1p: mp_ptr,
                           s2p: mp_srcptr,
                           s2n: mp_size_t,
                           s3limb: mp_limb_t)
                           -> mp_limb_t {
    mpn_divrem_1(r1p, 0, s2p, s2n, s3limb)
}
c_fn! {
    mpn_divexact_1 = __gmpn_divexact_1(rp: mp_ptr,
                                       sp: mp_srcptr,
                                       n: mp_size_t,
                                       d: mp_limb_t);
}
pub unsafe fn mpn_divexact_by3(rp: mp_ptr,
                               sp: mp_srcptr,
                               n: mp_size_t)
                               -> mp_limb_t {
    mpn_divexact_by3c(rp, sp, n, 0)
}
c_fn! {
    mpn_divexact_by3c = __gmpn_divexact_by3c(rp: mp_ptr,
                                             sp: mp_srcptr,
                                             n: mp_size_t,
                                             carry: mp_limb_t)
                                             -> mp_limb_t;
    mpn_mod_1 = __gmpn_mod_1(s1p: mp_srcptr,
                             s1n: mp_size_t,
                             s2limb: mp_limb_t)
                             -> mp_limb_t;
    mpn_lshift = __gmpn_lshift(rp: mp_ptr,
                               sp: mp_srcptr,
                               n: mp_size_t,
                               count: c_uint)
                               -> mp_limb_t;
    mpn_rshift = __gmpn_rshift(rp: mp_ptr,
                               sp: mp_srcptr,
                               n: mp_size_t,
                               count: c_uint)
                               -> mp_limb_t;
    mpn_cmp = __gmpn_cmp(s1p: mp_srcptr, s2p: mp_srcptr, n: mp_size_t) -> c_int;
    mpn_zero_p = __gmpn_zero_p(sp: mp_srcptr, n: mp_size_t) -> c_int;
    mpn_gcd = __gmpn_gcd(rp: mp_ptr,
                         xp: mp_ptr,
                         xn: mp_size_t,
                         yp: mp_ptr,
                         yn: mp_size_t)
                         -> mp_size_t;
    mpn_gcd_1 = __gmpn_gcd_1(xp: mp_srcptr,
                             xn: mp_size_t,
                             yimb: mp_limb_t)
                             -> mp_limb_t;
    mpn_gcdext = __gmpn_gcdext(gp: mp_ptr,
                               sp: mp_ptr,
                               sn: *mut mp_size_t,
                               up: mp_ptr,
                               un: mp_size_t,
                               vp: mp_ptr,
                               vn: mp_size_t)
                               -> mp_size_t;
    mpn_sqrtrem = __gmpn_sqrtrem(r1p: mp_ptr,
                                 r2p: mp_ptr,
                                 sp: mp_srcptr,
                                 n: mp_size_t)
                                 -> mp_size_t;
    mpn_sizeinbase = __gmpn_sizeinbase(xp: mp_srcptr,
                                       n: mp_size_t,
                                       base: c_int)
                                       -> usize;
    mpn_get_str = __gmpn_get_str(str: *mut c_uchar,
                                 base: c_int,
                                 s1p: mp_ptr,
                                 s1n: mp_size_t)
                                 -> usize;
    mpn_set_str = __gmpn_set_str(rp: mp_ptr,
                                 str: *const c_uchar,
                                 strsize: usize,
                                 base: c_int)
                                 -> mp_size_t;
    mpn_scan0 = __gmpn_scan0(s1p: mp_srcptr, bit: mp_bitcnt_t) -> mp_bitcnt_t;
    mpn_scan1 = __gmpn_scan1(s1p: mp_srcptr, bit: mp_bitcnt_t) -> mp_bitcnt_t;
    mpn_random = __gmpn_random(r1p: mp_ptr, r1n: mp_size_t);
    mpn_random2 = __gmpn_random2(r1p: mp_ptr, r1n: mp_size_t);
    mpn_popcount = __gmpn_popcount(s1p: mp_srcptr, n: mp_size_t) -> mp_bitcnt_t;
    mpn_hamdist = __gmpn_hamdist(s1p: mp_srcptr,
                                 s2p: mp_srcptr,
                                 n: mp_size_t)
                                 -> mp_bitcnt_t;
    mpn_perfect_square_p = __gmpn_perfect_square_p(s1p: mp_srcptr,
                                                   n: mp_size_t)
                                                   -> c_int;
    mpn_and_n = __gmpn_and_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t);
    mpn_ior_n = __gmpn_ior_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t);
    mpn_xor_n = __gmpn_xor_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: mp_size_t);
    mpn_andn_n = __gmpn_andn_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: mp_size_t);
    mpn_iorn_n = __gmpn_iorn_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: mp_size_t);
    mpn_nand_n = __gmpn_nand_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: mp_size_t);
    mpn_nior_n = __gmpn_nior_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: mp_size_t);
    mpn_xnor_n = __gmpn_xnor_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: mp_size_t);
    mpn_com = __gmpn_com(rp: mp_ptr, sp: mp_srcptr, n: mp_size_t);
    mpn_copyi = __gmpn_copyi(rp: mp_ptr, s1p: mp_srcptr, n: mp_size_t);
    mpn_copyd = __gmpn_copyd(rp: mp_ptr, s1p: mp_srcptr, n: mp_size_t);
    mpn_zero = __gmpn_zero(rp: mp_ptr, n: mp_size_t);

    // Low-level functions for cryptography
    mpn_cnd_add_n = __gmpn_cnd_add_n(cnd: mp_limb_t,
                                     rp: mp_ptr,
                                     s1p: mp_srcptr,
                                     s2p: mp_srcptr,
                                     n: mp_size_t)
                                     -> mp_limb_t;
    mpn_cnd_sub_n = __gmpn_cnd_sub_n(cnd: mp_limb_t,
                                     rp: mp_ptr,
                                     s1p: mp_srcptr,
                                     s2p: mp_srcptr,
                                     n: mp_size_t)
                                     -> mp_limb_t;
    mpn_sec_add_1 = __gmpn_sec_add_1(rp: mp_ptr,
                                     ap: mp_srcptr,
                                     n: mp_size_t,
                                     b: mp_limb_t,
                                     tp: mp_ptr)
                                     -> mp_limb_t;
    mpn_sec_add_1_itch = __gmpn_sec_add_1_itch(n: mp_size_t) -> mp_size_t;
    mpn_sec_sub_1 = __gmpn_sec_sub_1(rp: mp_ptr,
                                     ap: mp_srcptr,
                                     n: mp_size_t,
                                     b: mp_limb_t,
                                     tp: mp_ptr)
                                     -> mp_limb_t;
    mpn_sec_sub_1_itch = __gmpn_sec_sub_1_itch(n: mp_size_t) -> mp_size_t;
    mpn_cnd_swap = __gmpn_cnd_swap(cnd: mp_limb_t,
                                   ap: *mut mp_limb_t,
                                   bp: *mut mp_limb_t,
                                   n: mp_size_t);
    mpn_sec_mul = __gmpn_sec_mul(rp: mp_ptr,
                                 ap: mp_srcptr,
                                 an: mp_size_t,
                                 bp: mp_srcptr,
                                 bn: mp_size_t,
                                 tp: mp_ptr);
    mpn_sec_mul_itch = __gmpn_sec_mul_itch(an: mp_size_t,
                                           bn: mp_size_t)
                                           -> mp_size_t;
    mpn_sec_sqr = __gmpn_sec_sqr(rp: mp_ptr,
                                 ap: mp_srcptr,
                                 an: mp_size_t,
                                 tp: mp_ptr);
    mpn_sec_sqr_itch = __gmpn_sec_sqr_itch(an: mp_size_t) -> mp_size_t;
    mpn_sec_powm = __gmpn_sec_powm(rp: mp_ptr,
                                   bp: mp_srcptr,
                                   bn: mp_size_t,
                                   ep: mp_srcptr,
                                   enb: mp_bitcnt_t,
                                   mp: mp_srcptr,
                                   n: mp_size_t,
                                   tp: mp_ptr);
    mpn_sec_powm_itch = __gmpn_sec_powm_itch(bn: mp_size_t,
                                             enb: mp_bitcnt_t,
                                             n: mp_size_t)
                                             -> mp_size_t;
    mpn_sec_tabselect = __gmpn_sec_tabselect(rp: *mut mp_limb_t,
                                             tab: *const mp_limb_t,
                                             n: mp_size_t,
                                             nents: mp_size_t,
                                             which: mp_size_t);
    mpn_sec_div_qr = __gmpn_sec_div_qr(qp: mp_ptr,
                                       np: mp_ptr,
                                       nn: mp_size_t,
                                       dp: mp_srcptr,
                                       dn: mp_size_t,
                                       tp: mp_ptr)
                                       -> mp_limb_t;
    mpn_sec_div_qr_itch = __gmpn_sec_div_qr_itch(nn: mp_size_t,
                                                 dn: mp_size_t)
                                                 -> mp_size_t;
    mpn_sec_div_r = __gmpn_sec_div_r(np: mp_ptr,
                                     nn: mp_size_t,
                                     dp: mp_srcptr,
                                     dn: mp_size_t,
                                     tp: mp_ptr);
    mpn_sec_div_r_itch = __gmpn_sec_div_r_itch(nn: mp_size_t,
                                               dn: mp_size_t)
                                               -> mp_size_t;
    mpn_sec_invert = __gmpn_sec_invert(rp: mp_ptr,
                                       ap: mp_ptr,
                                       mp: mp_srcptr,
                                       n: mp_size_t,
                                       nbcnt: mp_bitcnt_t,
                                       tp: mp_ptr)
                                       -> c_int;
    mpn_sec_invert_itch = __gmpn_sec_invert_itch(n: mp_size_t) -> mp_size_t;
}

// Random Numbers

c_fn! {
    // Random State Initialization
    gmp_randinit_default = __gmp_randinit_default(state: randstate_ptr);
    gmp_randinit_mt = __gmp_randinit_mt(state: randstate_ptr);
    gmp_randinit_lc_2exp = __gmp_randinit_lc_2exp(state: randstate_ptr,
                                                 a: mpz_srcptr,
                                                 c: c_ulong,
                                                 m2exp: mp_bitcnt_t);
    gmp_randinit_lc_2exp_size = __gmp_randinit_lc_2exp_size(
        state: randstate_ptr,
        size: mp_bitcnt_t
    ) -> c_int;
    gmp_randinit_set = __gmp_randinit_set(rop: randstate_ptr,
                                          op: randstate_srcptr);
    gmp_randclear = __gmp_randclear(state: randstate_ptr);

    // Random State Seeding
    gmp_randseed = __gmp_randseed(state: randstate_ptr, seed: mpz_srcptr);
    gmp_randseed_ui = __gmp_randseed_ui(state: randstate_ptr, seed: c_ulong);

    // Random State Miscellaneous
    gmp_urandomb_ui = __gmp_urandomb_ui(state: randstate_ptr,
                                        n: c_ulong)
                                        -> c_ulong;
    gmp_urandomm_ui = __gmp_urandomm_ui(state: randstate_ptr,
                                        n: c_ulong)
                                        -> c_ulong;
}

// Formatted Output
c_fn! {
    gmp_printf = __gmp_printf(fmt: *const c_char; ...) -> c_int;
    gmp_sprintf = __gmp_sprintf(buf: *mut c_char,
                                fmt: *const c_char;
                                ...) -> c_int;
    gmp_snprintf = __gmp_snprintf(buf: *mut c_char,
                                  size: usize,
                                  fmt: *const c_char;
                                  ...)
                                  -> c_int;
    gmp_asprintf = __gmp_asprintf(pp: *mut *mut c_char,
                                  fmt: *const c_char;
                                  ...)
                                  -> c_int;
}

// Formatted Input
c_fn! {
    gmp_scanf = __gmp_scanf(fmt: *const c_char; ...) -> c_int;
    gmp_sscanf = __gmp_sscanf(s: *const c_char,
                              fmt: *const c_char;
                              ...) -> c_int;
}

// Custom Allocation
pub type allocate_function = Option<extern "C" fn(alloc_size: usize)
                                                  -> *mut c_void>;
pub type reallocate_function = Option<unsafe extern "C" fn(ptr: *mut c_void,
                                                           old_size: usize,
                                                           new_size: usize)
                                                           -> *mut c_void>;
pub type free_function = Option<unsafe extern "C" fn(ptr: *mut c_void,
                                                     size: usize)>;
c_fn! {
    mp_set_memory_functions = __gmp_set_memory_functions(
        alloc_func_ptr: allocate_function,
        realloc_func_ptr: reallocate_function,
        free_func_ptr: free_function
    );
    mp_get_memory_functions = __gmp_get_memory_functions(
        alloc_func_ptr: *mut allocate_function,
        realloc_func_ptr: *mut reallocate_function,
        free_func_ptr: *mut free_function
    );
}
