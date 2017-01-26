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

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

macro_rules! c_static {
    { $($c:tt $name:ident: $ty:ty;)* } => {
        $(
            #[link(name = "gmp", kind = "static")]
            extern "C" {
                #[link_name = $c]
                pub static $name: $ty;
            }
        )*
    };
}

macro_rules! c_fn {
    {
        $($c:tt $name:ident
          ($($par:ident: $ty:ty),* $(; $dots:tt)*) $(-> $ret:ty)*;
        )*
    } => {
        $(
            #[link(name = "gmp", kind = "static")]
            extern "C" {
                #[link_name = $c]
                pub fn $name($($par: $ty),* $(, $dots)*) $(-> $ret)*;
            }
        )*
    };
}

c_static! {
    "__gmp_bits_per_limb" bits_per_limb: c_int;
}
pub const VERSION: c_int = 6;
pub const VERSION_MINOR: c_int = 1;
pub const VERSION_PATCHLEVEL: c_int = 2;
c_static! {
    "__gmp_version" version: *const c_char;
}

pub type exp_t = c_long;
pub type limb_t = c_ulong;
pub type size_t = c_long;
pub type bitcnt_t = c_ulong;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpz_t {
    pub alloc: c_int,
    pub size: c_int,
    pub d: *mut limb_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpq_t {
    pub num: mpz_t,
    pub den: mpz_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpf_t {
    pub prec: c_int,
    pub size: c_int,
    pub exp: exp_t,
    pub d: *mut limb_t,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
enum randalg_t {
    _DEFAULT = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct randstate_t {
    seed: mpz_t,
    alg: randalg_t,
    algdata: *mut c_void,
}

// Types for function declarations in this file.

type mpz_srcptr = *const mpz_t;
type mpz_ptr = *mut mpz_t;
type mpq_srcptr = *const mpq_t;
type mpq_ptr = *mut mpq_t;
type mpf_srcptr = *const mpf_t;
type mpf_ptr = *mut mpf_t;
type mp_ptr = *mut limb_t;
type mp_srcptr = *const limb_t;
type randstate_srcptr = *const randstate_t;
type randstate_ptr = *mut randstate_t;

// Integers

c_fn! {
    // Initialization Functions
    "__gmpz_init" mpz_init(x: mpz_ptr);
    "__gmpz_inits" mpz_inits(x: mpz_ptr; ...);
    "__gmpz_init2" mpz_init2(x: mpz_ptr, n: bitcnt_t);
    "__gmpz_clear" mpz_clear(x: mpz_ptr);
    "__gmpz_clears" mpz_clears(x: mpz_ptr; ...);
    "__gmpz_realloc2" mpz_realloc2(x: mpz_ptr, n: bitcnt_t);

    // Assignment Functions
    "__gmpz_set" mpz_set(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_set_ui" mpz_set_ui(rop: mpz_ptr, op: c_ulong);
    "__gmpz_set_si" mpz_set_si(rop: mpz_ptr, op: c_long);
    "__gmpz_set_d" mpz_set_d(rop: mpz_ptr, op: f64);
    "__gmpz_set_q" mpz_set_q(rop: mpz_ptr, op: mpq_srcptr);
    "__gmpz_set_f" mpz_set_f(rop: mpz_ptr, op: mpf_srcptr);
    "__gmpz_set_str" mpz_set_str(rop: mpz_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    "__gmpz_swap" mpz_swap(rop1: mpz_ptr, rop2: mpz_ptr);

    // Combined Initialization and Assignment Functions
    "__gmpz_init_set" mpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_init_set_ui" mpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    "__gmpz_init_set_si" mpz_init_set_si(rop: mpz_ptr, op: c_long);
    "__gmpz_init_set_d" mpz_init_set_d(rop: mpz_ptr, op: f64);
    "__gmpz_init_set_str" mpz_init_set_str(rop: mpz_ptr,
                                           str: *const c_char,
                                           base: c_int)
                                           -> c_int;

    // Conversion Functions
    "__gmpz_get_ui" mpz_get_ui(op: mpz_srcptr) -> c_ulong;
    "__gmpz_get_si" mpz_get_si(op: mpz_srcptr) -> c_long;
    "__gmpz_get_d" mpz_get_d(op: mpz_srcptr) -> f64;
    "__gmpz_get_d_2exp" mpz_get_d_2exp(exp: *mut c_long, op: mpz_srcptr) -> f64;
    "__gmpz_get_str" mpz_get_str(str: *mut c_char,
                                 base: c_int,
                                 op: mpz_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    "__gmpz_add" mpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_add_ui" mpz_add_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    "__gmpz_sub" mpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_sub_ui" mpz_sub_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    "__gmpz_ui_sub" mpz_ui_sub(rop: mpz_ptr, op1: c_ulong, op2: mpz_srcptr);
    "__gmpz_mul" mpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_mul_si" mpz_mul_si(rop: mpz_ptr, op1: mpz_srcptr, op2: c_long);
    "__gmpz_mul_ui" mpz_mul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    "__gmpz_addmul" mpz_addmul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_addmul_ui" mpz_addmul_ui(rop: mpz_ptr,
                                     op1: mpz_srcptr,
                                     op2: c_ulong);
    "__gmpz_submul" mpz_submul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_submul_ui" mpz_submul_ui(rop: mpz_ptr,
                                     op1: mpz_srcptr,
                                     op2: c_ulong);
    "__gmpz_mul_2exp" mpz_mul_2exp(rop: mpz_ptr,
                                   op1: mpz_srcptr,
                                   op2: bitcnt_t);
    "__gmpz_neg" mpz_neg(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_abs" mpz_abs(rop: mpz_ptr, op: mpz_srcptr);

    // Division Functions
    "__gmpz_cdiv_q" mpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_cdiv_r" mpz_cdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_cdiv_qr" mpz_cdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    "__gmpz_cdiv_q_ui" mpz_cdiv_q_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_cdiv_r_ui" mpz_cdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_cdiv_qr_ui" mpz_cdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    "__gmpz_cdiv_ui" mpz_cdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    "__gmpz_cdiv_q_2exp" mpz_cdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_cdiv_r_2exp" mpz_cdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_fdiv_q" mpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_fdiv_r" mpz_fdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_fdiv_qr" mpz_fdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    "__gmpz_fdiv_q_ui" mpz_fdiv_q_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_fdiv_r_ui" mpz_fdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_fdiv_qr_ui" mpz_fdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    "__gmpz_fdiv_ui" mpz_fdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    "__gmpz_fdiv_q_2exp" mpz_fdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_fdiv_r_2exp" mpz_fdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_tdiv_q" mpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_tdiv_r" mpz_tdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_tdiv_qr" mpz_tdiv_qr(q: mpz_ptr,
                                 r: mpz_ptr,
                                 n: mpz_srcptr,
                                 d: mpz_srcptr);
    "__gmpz_tdiv_q_ui" mpz_tdiv_q_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_tdiv_r_ui" mpz_tdiv_r_ui(q: mpz_ptr,
                                     n: mpz_srcptr,
                                     d: c_ulong)
                                     -> c_ulong;
    "__gmpz_tdiv_qr_ui" mpz_tdiv_qr_ui(q: mpz_ptr,
                                       r: mpz_ptr,
                                       n: mpz_srcptr,
                                       d: c_ulong)
                                       -> c_ulong;
    "__gmpz_tdiv_ui" mpz_tdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    "__gmpz_tdiv_q_2exp" mpz_tdiv_q_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_tdiv_r_2exp" mpz_tdiv_r_2exp(q: mpz_ptr,
                                         n: mpz_srcptr,
                                         b: bitcnt_t);
    "__gmpz_mod" mpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
}
pub use self::mpz_fdiv_r_ui as mpz_mod_ui;
c_fn! {
    "__gmpz_divexact" mpz_divexact(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    "__gmpz_divexact_ui" mpz_divexact_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong);
    "__gmpz_divisible_p" mpz_divisible_p(n: mpz_srcptr, d: mpz_srcptr) -> c_int;
    "__gmpz_divisible_ui_p" mpz_divisible_ui_p(n: mpz_srcptr,
                                               d: c_ulong)
                                               -> c_int;
    "__gmpz_divisible_2exp_p" mpz_divisible_2exp_p(n: mpz_srcptr,
                                                   b: bitcnt_t)
                                                   -> c_int;
    "__gmpz_congruent_p" mpz_congruent_p(n: mpz_srcptr,
                                         c: mpz_srcptr,
                                         d: mpz_srcptr)
                                         -> c_int;
    "__gmpz_congruent_ui_p" mpz_congruent_ui_p(n: mpz_srcptr,
                                               c: c_ulong,
                                               d: c_ulong)
                                               -> c_int;
    "__gmpz_congruent_2exp_p" mpz_congruent_2exp_p(n: mpz_srcptr,
                                                   c: mpz_srcptr,
                                                   b: bitcnt_t)
                                                   -> c_int;

    // Exponentiation Functions
    "__gmpz_powm" mpz_powm(rop: mpz_ptr,
                           base: mpz_srcptr,
                           exp: mpz_srcptr,
                           modu: mpz_srcptr);
    "__gmpz_powm_ui" mpz_powm_ui(rop: mpz_ptr,
                                 base: mpz_srcptr,
                                 exp: c_ulong,
                                 modu: mpz_srcptr);
    "__gmpz_powm_sec" mpz_powm_sec(rop: mpz_ptr,
                                   base: mpz_srcptr,
                                   exp: mpz_srcptr,
                                   modu: mpz_srcptr);
    "__gmpz_pow_ui" mpz_pow_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong);
    "__gmpz_ui_pow_ui" mpz_ui_pow_ui(rop: mpz_ptr, base: c_ulong, exp: c_ulong);

    // Root Extraction Functions
    "__gmpz_root" mpz_root(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong) -> c_int;
    "__gmpz_rootrem" mpz_rootrem(root: mpz_ptr,
                                 rem: mpz_ptr,
                                 op: mpz_srcptr,
                                 n: c_ulong);
    "__gmpz_sqrt" mpz_sqrt(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_sqrtrem" mpz_sqrtrem(rop1: mpz_ptr, rop2: mpz_ptr, op: mpz_srcptr);
    "__gmpz_perfect_power_p" mpz_perfect_power_p(op: mpz_srcptr) -> c_int;
    "__gmpz_perfect_square_p" mpz_perfect_square_p(op: mpz_srcptr) -> c_int;

    // Number Theoretic Functions
    "__gmpz_probab_prime_p" mpz_probab_prime_p(n: mpz_srcptr,
                                               reps: c_int)
                                               -> c_int;
    "__gmpz_nextprime" mpz_nextprime(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_gcd" mpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_gcd_ui" mpz_gcd_ui(rop: mpz_ptr,
                               op1: mpz_srcptr,
                               op2: c_ulong)
                               -> c_ulong;
    "__gmpz_gcdext" mpz_gcdext(g: mpz_ptr,
                               s: mpz_ptr,
                               t: mpz_ptr,
                               a: mpz_srcptr,
                               b: mpz_srcptr);
    "__gmpz_lcm" mpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_lcm_ui" mpz_lcm_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    "__gmpz_invert" mpz_invert(rop: mpz_ptr,
                               op1: mpz_srcptr,
                               op2: mpz_srcptr)
                               -> c_int;
    "__gmpz_jacobi" mpz_jacobi(a: mpz_srcptr, b: mpz_srcptr) -> c_int;
}
pub use self::mpz_jacobi as mpz_legendre;
pub use self::mpz_jacobi as mpz_kronecker;
c_fn! {
    "__gmpz_kronecker_si" mpz_kronecker_si(a: mpz_srcptr, b: c_long) -> c_int;
    "__gmpz_kronecker_ui" mpz_kronecker_ui(a: mpz_srcptr, b: c_ulong) -> c_int;
    "__gmpz_si_kronecker" mpz_si_kronecker(a: c_long, b: mpz_srcptr) -> c_int;
    "__gmpz_ui_kronecker" mpz_ui_kronecker(a: c_ulong, b: mpz_srcptr) -> c_int;
    "__gmpz_remove" mpz_remove(rop: mpz_ptr,
                               op: mpz_srcptr,
                               f: mpz_srcptr)
                               -> bitcnt_t;
    "__gmpz_fac_ui" mpz_fac_ui(rop: mpz_ptr, n: c_ulong);
    "__gmpz_2fac_ui" mpz_2fac_ui(rop: mpz_ptr, n: c_ulong);
    "__gmpz_mfac_uiui" mpz_mfac_uiui(rop: mpz_ptr, n: c_ulong, m: c_ulong);
    "__gmpz_primorial_ui" mpz_primorial_ui(r: mpz_ptr, n: c_ulong);
    "__gmpz_bin_ui" mpz_bin_ui(rop: mpz_ptr, n: mpz_srcptr, k: c_ulong);
    "__gmpz_bin_uiui" mpz_bin_uiui(rop: mpz_ptr, n: c_ulong, k: c_ulong);
    "__gmpz_fib_ui" mpz_fib_ui(f_n: mpz_ptr, n: c_ulong);
    "__gmpz_fib2_ui" mpz_fib2_ui(f_n: mpz_ptr, fnsub1: mpz_ptr, n: c_ulong);
    "__gmpz_lucnum_ui" mpz_lucnum_ui(ln: mpz_ptr, n: c_ulong);
    "__gmpz_lucnum2_ui" mpz_lucnum2_ui(ln: mpz_ptr,
                                       lnsub1: mpz_ptr,
                                       n: c_ulong);

    // Comparison Functions
    "__gmpz_cmp" mpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    "__gmpz_cmp_d" mpz_cmp_d(op1: mpz_srcptr, op2: f64) -> c_int;
    "__gmpz_cmp_si" mpz_cmp_si(op1: mpz_srcptr, op2: c_long) -> c_int;
    "__gmpz_cmp_ui" mpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    "__gmpz_cmpabs" mpz_cmpabs(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    "__gmpz_cmpabs_d" mpz_cmpabs_d(op1: mpz_srcptr, op2: f64) -> c_int;
    "__gmpz_cmpabs_ui" mpz_cmpabs_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
}
#[inline]
pub unsafe fn mpz_sgn(op: mpz_srcptr) -> c_int {
    if (*op).size < 0 {
        -1
    } else if (*op).size > 0 {
        1
    } else {
        0
    }
}
c_fn! {
    "__gmpz_and" mpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_ior" mpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_xor" mpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    "__gmpz_com" mpz_com(rop: mpz_ptr, op: mpz_srcptr);
    "__gmpz_popcount" mpz_popcount(op: mpz_srcptr) -> bitcnt_t;
    "__gmpz_hamdist" mpz_hamdist(op1: mpz_srcptr, op2: mpz_srcptr) -> bitcnt_t;
    "__gmpz_scan0" mpz_scan0(op: mpz_srcptr,
                             starting_bit: bitcnt_t)
                             -> bitcnt_t;
    "__gmpz_scan1" mpz_scan1(op: mpz_srcptr,
                             starting_bit: bitcnt_t)
                             -> bitcnt_t;
    "__gmpz_setbit" mpz_setbit(rop: mpz_ptr, bit_index: bitcnt_t);
    "__gmpz_clrbit" mpz_clrbit(rop: mpz_ptr, bit_index: bitcnt_t);
    "__gmpz_combit" mpz_combit(rop: mpz_ptr, bit_index: bitcnt_t);
    "__gmpz_tstbit" mpz_tstbit(rop: mpz_srcptr, bit_index: bitcnt_t) -> c_int;

    // Random Number Functions
    "__gmpz_urandomb" mpz_urandomb(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: bitcnt_t);
    "__gmpz_urandomm" mpz_urandomm(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: mpz_srcptr);
    "__gmpz_rrandomb" mpz_rrandomb(rop: mpz_ptr,
                                   state: randstate_ptr,
                                   n: bitcnt_t);
    "__gmpz_random2" mpz_random2(rop: mpz_ptr, max_size: size_t);

    // Integer Import and Export
    "__gmpz_import" mpz_import(rop: mpz_ptr,
                               count: usize,
                               order: c_int,
                               size: usize,
                               endian: c_int,
                               nails: usize,
                               op: *const c_void);
    "__gmpz_export" mpz_export(rop: *mut c_void,
                               countp: *mut usize,
                               order: c_int,
                               size: usize,
                               endian: c_int,
                               nails: usize,
                               op: mpz_srcptr)
                               -> *mut c_void;

    // Miscellaneous Functions
    "__gmpz_fits_ulong_p" mpz_fits_ulong_p(op: mpz_srcptr) -> c_int;
    "__gmpz_fits_slong_p" mpz_fits_slong_p(op: mpz_srcptr) -> c_int;
    "__gmpz_fits_uint_p" mpz_fits_uint_p(op: mpz_srcptr) -> c_int;
    "__gmpz_fits_sint_p" mpz_fits_sint_p(op: mpz_srcptr) -> c_int;
    "__gmpz_fits_ushort_p" mpz_fits_ushort_p(op: mpz_srcptr) -> c_int;
    "__gmpz_fits_sshort_p" mpz_fits_sshort_p(op: mpz_srcptr) -> c_int;
}
#[inline]
pub unsafe fn mpz_odd_p(op: mpz_srcptr) -> c_int {
    (*(*op).d) as c_int & if (*op).size != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn mpz_even_p(op: mpz_srcptr) -> c_int {
    !mpz_odd_p(op)
}
c_fn! {
    "__gmpz_sizeinbase" mpz_sizeinbase(arg1: mpz_srcptr, arg2: c_int) -> usize;

    // Special Functions
    "__gmpz_realloc" _mpz_realloc(integer: mpz_ptr,
                                  new_alloc: size_t)
                                  -> *mut c_void;
    "__gmpz_getlimbn" mpz_getlimbn(op: mpz_srcptr, n: size_t) -> limb_t;
    "__gmpz_size" mpz_size(op: mpz_srcptr) -> usize;
    "__gmpz_limbs_read" mpz_limbs_read(x: mpz_srcptr) -> mp_srcptr;
    "__gmpz_limbs_write" mpz_limbs_write(x: mpz_ptr, n: size_t) -> mp_ptr;
    "__gmpz_limbs_modify" mpz_limbs_modify(x: mpz_ptr, n: size_t) -> mp_ptr;
    "__gmpz_limbs_finish" mpz_limbs_finish(x: mpz_ptr, s: size_t);
    "__gmpz_roinit_n" mpz_roinit_n(x: mpz_ptr,
                                   xp: mp_srcptr,
                                   xs: size_t)
                                   -> mpz_srcptr;

    // Rational numbers
    "__gmpq_canonicalize" mpq_canonicalize(op: mpq_ptr);

    // Initialization and Assignment Functions
    "__gmpq_init" mpq_init(x: mpq_ptr);
    "__gmpq_inits" mpq_inits(x: mpq_ptr; ...);
    "__gmpq_clear" mpq_clear(x: mpq_ptr);
    "__gmpq_clears" mpq_clears(x: mpq_ptr; ...);
    "__gmpq_set" mpq_set(rop: mpq_ptr, op: mpq_srcptr);
    "__gmpq_set_z" mpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    "__gmpq_set_ui" mpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    "__gmpq_set_si" mpq_set_si(rop: mpq_ptr, op1: c_long, op2: c_ulong);
    "__gmpq_set_str" mpq_set_str(rop: mpq_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    "__gmpq_swap" mpq_swap(rop1: mpq_ptr, rop2: mpq_ptr);

    // Conversion Functions
    "__gmpq_get_d" mpq_get_d(op: mpq_srcptr) -> f64;
    "__gmpq_set_d" mpq_set_d(rop: mpq_ptr, op: f64);
    "__gmpq_set_f" mpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    "__gmpq_get_str" mpq_get_str(str: *mut c_char,
                                 base: c_int,
                                 op: mpq_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    "__gmpq_add" mpq_add(sum: mpq_ptr,
                         addend1: mpq_srcptr,
                         addend2: mpq_srcptr);
    "__gmpq_sub" mpq_sub(difference: mpq_ptr,
                         minuend: mpq_srcptr,
                         subtrahend: mpq_srcptr);
    "__gmpq_mul" mpq_mul(product: mpq_ptr,
                         multiplier: mpq_srcptr,
                         multiplicand: mpq_srcptr);
    "__gmpq_mul_2exp" mpq_mul_2exp(rop: mpq_ptr,
                                   op1: mpq_srcptr,
                                   op2: bitcnt_t);
    "__gmpq_div" mpq_div(quotient: mpq_ptr,
                         dividend: mpq_srcptr,
                         divisor: mpq_srcptr);
    "__gmpq_div_2exp" mpq_div_2exp(rop: mpq_ptr,
                                   op1: mpq_srcptr,
                                   op2: bitcnt_t);
    "__gmpq_neg" mpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr);
    "__gmpq_abs" mpq_abs(rop: mpq_ptr, op: mpq_srcptr);
    "__gmpq_inv" mpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);

    // Comparison Functions
    "__gmpq_cmp" mpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    "__gmpq_cmp_z" mpq_cmp_z(op1: mpq_srcptr, op2: mpz_srcptr) -> c_int;
    "__gmpq_cmp_ui" mpq_cmp_ui(op1: mpq_srcptr,
                               num2: c_ulong,
                               den2: c_ulong)
                               -> c_int;
    "__gmpq_cmp_si" mpq_cmp_si(op1: mpq_srcptr,
                               num2: c_long,
                               den2: c_ulong)
                               -> c_int;
}
#[inline]
pub unsafe fn mpq_sgn(op: mpq_srcptr) -> c_int {
    if (*op).num.size < 0 {
        -1
    } else if (*op).num.size > 0 {
        1
    } else {
        0
    }
}
c_fn! {
    "__gmpq_equal" mpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;

    // Applying Integer Functions to Rationals
}
#[inline]
pub unsafe fn mpq_numref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op).num) as mpz_ptr
}
#[inline]
pub unsafe fn mpq_denref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op).den) as mpz_ptr
}
c_fn! {
    "__gmpq_get_num" mpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    "__gmpq_get_den" mpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    "__gmpq_set_den" mpq_set_den(rational: mpq_ptr, numerator: mpz_srcptr);
    "__gmpq_set_num" mpq_set_num(rational: mpq_ptr, denominator: mpz_srcptr);
}

// Floating-point numbers

c_fn! {
    // Initialization Functions
    "__gmpf_set_default_prec" mpf_set_default_prec(prec: bitcnt_t);
    "__gmpf_get_default_prec" mpf_get_default_prec() -> bitcnt_t;
    "__gmpf_init" mpf_init(x: mpf_ptr);
    "__gmpf_init2" mpf_init2(x: mpf_ptr, prec: bitcnt_t);
    "__gmpf_inits" mpf_inits(x: mpf_ptr; ...);
    "__gmpf_clear" mpf_clear(x: mpf_ptr);
    "__gmpf_clears" mpf_clears(x: mpf_ptr; ...);
    "__gmpf_get_prec" mpf_get_prec(op: mpf_srcptr) -> bitcnt_t;
    "__gmpf_set_prec" mpf_set_prec(rop: mpf_ptr, prec: bitcnt_t);
    "__gmpf_set_prec_raw" mpf_set_prec_raw(rop: mpf_ptr, prec: bitcnt_t);

    // Assignment Functions
    "__gmpf_set" mpf_set(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_set_ui" mpf_set_ui(rop: mpf_ptr, op: c_ulong);
    "__gmpf_set_si" mpf_set_si(rop: mpf_ptr, op: c_long);
    "__gmpf_set_d" mpf_set_d(rop: mpf_ptr, op: f64);
    "__gmpf_set_z" mpf_set_z(rop: mpf_ptr, op: mpz_srcptr);
    "__gmpf_set_q" mpf_set_q(rop: mpf_ptr, op: mpq_srcptr);
    "__gmpf_set_str" mpf_set_str(rop: mpf_ptr,
                                 str: *const c_char,
                                 base: c_int)
                                 -> c_int;
    "__gmpf_swap" mpf_swap(rop1: mpf_ptr, rop2: mpf_ptr);

    // Combined Initialization and Assignment Functions
    "__gmpf_init_set" mpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_init_set_ui" mpf_init_set_ui(rop: mpf_ptr, op: c_ulong);
    "__gmpf_init_set_si" mpf_init_set_si(rop: mpf_ptr, op: c_long);
    "__gmpf_init_set_d" mpf_init_set_d(rop: mpf_ptr, op: f64);
    "__gmpf_init_set_str" mpf_init_set_str(rop: mpf_ptr,
                                           str: *const c_char,
                                           base: c_int)
                                           -> c_int;

    // Conversion Functions
    "__gmpf_get_d" mpf_get_d(op: mpf_srcptr) -> f64;
    "__gmpf_get_d_2exp" mpf_get_d_2exp(exp: *mut c_long, op: mpf_srcptr) -> f64;
    "__gmpf_get_si" mpf_get_si(op: mpf_srcptr) -> c_long;
    "__gmpf_get_ui" mpf_get_ui(op: mpf_srcptr) -> c_ulong;
    "__gmpf_get_str" mpf_get_str(str: *mut c_char,
                                 expptr: *mut exp_t,
                                 base: c_int,
                                 n_digits: usize,
                                 op: mpf_srcptr)
                                 -> *mut c_char;

    // Arithmetic Functions
    "__gmpf_add" mpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    "__gmpf_add_ui" mpf_add_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    "__gmpf_sub" mpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    "__gmpf_ui_sub" mpf_ui_sub(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    "__gmpf_sub_ui" mpf_sub_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    "__gmpf_mul" mpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    "__gmpf_mul_ui" mpf_mul_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    "__gmpf_div" mpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    "__gmpf_ui_div" mpf_ui_div(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    "__gmpf_div_ui" mpf_div_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    "__gmpf_sqrt" mpf_sqrt(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_sqrt_ui" mpf_sqrt_ui(rop: mpf_ptr, op: c_ulong);
    "__gmpf_pow_ui" mpf_pow_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    "__gmpf_neg" mpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_abs" mpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_mul_2exp" mpf_mul_2exp(rop: mpf_ptr,
                                   op1: mpf_srcptr,
                                   op2: bitcnt_t);
    "__gmpf_div_2exp" mpf_div_2exp(rop: mpf_ptr,
                                   op1: mpf_srcptr,
                                   op2: bitcnt_t);

    // Comparison Functions
    "__gmpf_cmp" mpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    "__gmpf_cmp_z" mpf_cmp_z(op1: mpf_srcptr, op2: mpz_srcptr) -> c_int;
    "__gmpf_cmp_d" mpf_cmp_d(op1: mpf_srcptr, op2: f64) -> c_int;
    "__gmpf_cmp_ui" mpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    "__gmpf_cmp_si" mpf_cmp_si(op1: mpf_srcptr, op2: c_long) -> c_int;
    "__gmpf_eq" mpf_eq(op1: mpf_srcptr,
                       op2: mpf_srcptr,
                       op3: bitcnt_t)
                       -> c_int;
    "__gmpf_reldiff" mpf_reldiff(rop: mpf_ptr,
                                 op1: mpf_srcptr,
                                 op2: mpf_srcptr);
}
#[inline]
pub unsafe fn mpf_sgn(op: mpf_srcptr) -> c_int {
    if (*op).size < 0 {
        -1
    } else if (*op).size > 0 {
        1
    } else {
        0
    }
}
c_fn! {

    // Miscellaneous Functions
    "__gmpf_ceil" mpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_floor" mpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_trunc" mpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
    "__gmpf_integer_p" mpf_integer_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_ulong_p" mpf_fits_ulong_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_slong_p" mpf_fits_slong_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_uint_p" mpf_fits_uint_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_sint_p" mpf_fits_sint_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_ushort_p" mpf_fits_ushort_p(op: mpf_srcptr) -> c_int;
    "__gmpf_fits_sshort_p" mpf_fits_sshort_p(op: mpf_srcptr) -> c_int;
    "__gmpf_urandomb" mpf_urandomb(rop: mpf_t,
                                   state: randstate_ptr,
                                   nbits: bitcnt_t);
    "__gmpf_random2" mpf_random2(rop: mpf_ptr, max_size: size_t, exp: exp_t);
}

// Low-Level Functions

c_fn! {
    "__gmpn_add_n" mpn_add_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t)
                             -> limb_t;
    "__gmpn_add_1" mpn_add_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: size_t,
                             s2limb: limb_t)
                             -> limb_t;
    "__gmpn_add" mpn_add(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: size_t,
                         s2p: mp_srcptr,
                         s2n: size_t)
                         -> limb_t;
    "__gmpn_sub_n" mpn_sub_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t)
                             -> limb_t;
    "__gmpn_sub_1" mpn_sub_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: size_t,
                             s2limb: limb_t)
                             -> limb_t;
    "__gmpn_sub" mpn_sub(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: size_t,
                         s2p: mp_srcptr,
                         s2n: size_t)
                         -> limb_t;
    "__gmpn_neg" mpn_neg(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t;
    "__gmpn_mul_n" mpn_mul_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t);
    "__gmpn_mul" mpn_mul(rp: mp_ptr,
                         s1p: mp_srcptr,
                         s1n: size_t,
                         s2p: mp_srcptr,
                         s2n: size_t)
                         -> limb_t;
    "__gmpn_sqr" mpn_sqr(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    "__gmpn_mul_1" mpn_mul_1(rp: mp_ptr,
                             s1p: mp_srcptr,
                             n: size_t,
                             s2limb: limb_t)
                             -> limb_t;
    "__gmpn_addmul_1" mpn_addmul_1(rp: mp_ptr,
                                   s1p: mp_srcptr,
                                   n: size_t,
                                   s2limb: limb_t)
                                   -> limb_t;
    "__gmpn_submul_1" mpn_submul_1(rp: mp_ptr,
                                   s1p: mp_srcptr,
                                   n: size_t,
                                   s2limb: limb_t)
                                   -> limb_t;
    "__gmpn_tdiv_qr" mpn_tdiv_qr(qp: mp_ptr,
                                 rp: mp_ptr,
                                 qxn: size_t,
                                 np: mp_srcptr,
                                 nn: size_t,
                                 dp: mp_srcptr,
                                 dn: size_t);
    "__gmpn_divrem_1" mpn_divrem_1(r1p: mp_ptr,
                                   qxn: size_t,
                                   s2p: mp_srcptr,
                                   s2n: size_t,
                                   s3limb: limb_t)
                                   -> limb_t;
}
#[inline]
pub unsafe fn mpn_divmod_1(r1p: mp_ptr,
                           s2p: mp_srcptr,
                           s2n: size_t,
                           s3limb: limb_t)
                           -> limb_t {
    mpn_divrem_1(r1p, 0, s2p, s2n, s3limb)
}
c_fn! {
    "__gmpn_divexact_1" mpn_divexact_1(rp: mp_ptr,
                                       sp: mp_srcptr,
                                       n: size_t,
                                       d: limb_t);
}
pub unsafe fn mpn_divexact_by3(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t {
    mpn_divexact_by3c(rp, sp, n, 0)
}
c_fn! {
    "__gmpn_divexact_by3c" mpn_divexact_by3c(rp: mp_ptr,
                                             sp: mp_srcptr,
                                             n: size_t,
                                             carry: limb_t)
                                             -> limb_t;
    "__gmpn_mod_1" mpn_mod_1(s1p: mp_srcptr,
                             s1n: size_t,
                             s2limb: limb_t)
                             -> limb_t;
    "__gmpn_lshift" mpn_lshift(rp: mp_ptr,
                               sp: mp_srcptr,
                               n: size_t,
                               count: c_uint)
                               -> limb_t;
    "__gmpn_rshift" mpn_rshift(rp: mp_ptr,
                               sp: mp_srcptr,
                               n: size_t,
                               count: c_uint)
                               -> limb_t;
    "__gmpn_cmp" mpn_cmp(s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> c_int;
    "__gmpn_zero_p" mpn_zero_p(sp: mp_srcptr, n: size_t) -> c_int;
    "__gmpn_gcd" mpn_gcd(rp: mp_ptr,
                         xp: mp_ptr,
                         xn: size_t,
                         yp: mp_ptr,
                         yn: size_t)
                         -> size_t;
    "__gmpn_gcd_1" mpn_gcd_1(xp: mp_srcptr, xn: size_t, yimb: limb_t) -> limb_t;
    "__gmpn_gcdext" mpn_gcdext(gp: mp_ptr,
                               sp: mp_ptr,
                               sn: *mut size_t,
                               up: mp_ptr,
                               un: size_t,
                               vp: mp_ptr,
                               vn: size_t)
                               -> size_t;
    "__gmpn_sqrtrem" mpn_sqrtrem(r1p: mp_ptr,
                                 r2p: mp_ptr,
                                 sp: mp_srcptr,
                                 n: size_t)
                                 -> size_t;
    "__gmpn_sizeinbase" mpn_sizeinbase(xp: mp_srcptr,
                                       n: size_t,
                                       base: c_int)
                                       -> usize;
    "__gmpn_get_str" mpn_get_str(str: *mut c_uchar,
                                 base: c_int,
                                 s1p: mp_ptr,
                                 s1n: size_t)
                                 -> usize;
    "__gmpn_set_str" mpn_set_str(rp: mp_ptr,
                                 str: *const c_uchar,
                                 strsize: usize,
                                 base: c_int)
                                 -> size_t;
    "__gmpn_scan0" mpn_scan0(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    "__gmpn_scan1" mpn_scan1(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    "__gmpn_random" mpn_random(r1p: mp_ptr, r1n: size_t);
    "__gmpn_random2" mpn_random2(r1p: mp_ptr, r1n: size_t);
    "__gmpn_popcount" mpn_popcount(s1p: mp_srcptr, n: size_t) -> bitcnt_t;
    "__gmpn_hamdist" mpn_hamdist(s1p: mp_srcptr,
                                 s2p: mp_srcptr,
                                 n: size_t)
                                 -> bitcnt_t;
    "__gmpn_perfect_square_p" mpn_perfect_square_p(s1p: mp_srcptr,
                                                   n: size_t)
                                                   -> c_int;
    "__gmpn_and_n" mpn_and_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t);
    "__gmpn_ior_n" mpn_ior_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t);
    "__gmpn_xor_n" mpn_xor_n(rp: mp_ptr,
                             s1p: mp_srcptr,
                             s2p: mp_srcptr,
                             n: size_t);
    "__gmpn_andn_n" mpn_andn_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: size_t);
    "__gmpn_iorn_n" mpn_iorn_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: size_t);
    "__gmpn_nand_n" mpn_nand_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: size_t);
    "__gmpn_nior_n" mpn_nior_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: size_t);
    "__gmpn_xnor_n" mpn_xnor_n(rp: mp_ptr,
                               s1p: mp_srcptr,
                               s2p: mp_srcptr,
                               n: size_t);
    "__gmpn_com" mpn_com(rp: mp_ptr, sp: mp_srcptr, n: size_t);
    "__gmpn_copyi" mpn_copyi(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    "__gmpn_copyd" mpn_copyd(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    "__gmpn_zero" mpn_zero(rp: mp_ptr, n: size_t);

    // Low-level functions for cryptography
    "__gmpn_cnd_add_n" mpn_cnd_add_n(cnd: limb_t,
                                     rp: mp_ptr,
                                     s1p: mp_srcptr,
                                     s2p: mp_srcptr,
                                     n: size_t)
                                     -> limb_t;
    "__gmpn_cnd_sub_n" mpn_cnd_sub_n(cnd: limb_t,
                                     rp: mp_ptr,
                                     s1p: mp_srcptr,
                                     s2p: mp_srcptr,
                                     n: size_t)
                                     -> limb_t;
    "__gmpn_sec_add_1" mpn_sec_add_1(rp: mp_ptr,
                                     ap: mp_srcptr,
                                     n: size_t,
                                     b: limb_t,
                                     tp: mp_ptr)
                                     -> limb_t;
    "__gmpn_sec_add_1_itch" mpn_sec_add_1_itch(n: size_t) -> size_t;
    "__gmpn_sec_sub_1" mpn_sec_sub_1(rp: mp_ptr,
                                     ap: mp_srcptr,
                                     n: size_t,
                                     b: limb_t,
                                     tp: mp_ptr)
                                     -> limb_t;
    "__gmpn_sec_sub_1_itch" mpn_sec_sub_1_itch(n: size_t) -> size_t;
    "__gmpn_cnd_swap" mpn_cnd_swap(cnd: limb_t,
                                   ap: *mut limb_t,
                                   bp: *mut limb_t,
                                   n: size_t);
    "__gmpn_sec_mul" mpn_sec_mul(rp: mp_ptr,
                                 ap: mp_srcptr,
                                 an: size_t,
                                 bp: mp_srcptr,
                                 bn: size_t,
                                 tp: mp_ptr);
    "__gmpn_sec_mul_itch" mpn_sec_mul_itch(an: size_t, bn: size_t) -> size_t;
    "__gmpn_sec_sqr" mpn_sec_sqr(rp: mp_ptr,
                                 ap: mp_srcptr,
                                 an: size_t,
                                 tp: mp_ptr);
    "__gmpn_sec_sqr_itch" mpn_sec_sqr_itch(an: size_t) -> size_t;
    "__gmpn_sec_powm" mpn_sec_powm(rp: mp_ptr,
                                   bp: mp_srcptr,
                                   bn: size_t,
                                   ep: mp_srcptr,
                                   enb: bitcnt_t,
                                   mp: mp_srcptr,
                                   n: size_t,
                                   tp: mp_ptr);
    "__gmpn_sec_powm_itch" mpn_sec_powm_itch(bn: size_t,
                                             enb: bitcnt_t,
                                             n: size_t)
                                             -> size_t;
    "__gmpn_sec_tabselect" mpn_sec_tabselect(rp: *mut limb_t,
                                             tab: *const limb_t,
                                             n: size_t,
                                             nents: size_t,
                                             which: size_t);
    "__gmpn_sec_div_qr" mpn_sec_div_qr(qp: mp_ptr,
                                       np: mp_ptr,
                                       nn: size_t,
                                       dp: mp_srcptr,
                                       dn: size_t,
                                       tp: mp_ptr)
                                       -> limb_t;
    "__gmpn_sec_div_qr_itch" mpn_sec_div_qr_itch(nn: size_t,
                                                 dn: size_t)
                                                 -> size_t;
    "__gmpn_sec_div_r" mpn_sec_div_r(np: mp_ptr,
                                     nn: size_t,
                                     dp: mp_srcptr,
                                     dn: size_t,
                                     tp: mp_ptr);
    "__gmpn_sec_div_r_itch" mpn_sec_div_r_itch(nn: size_t,
                                               dn: size_t)
                                               -> size_t;
    "__gmpn_sec_invert" mpn_sec_invert(rp: mp_ptr,
                                       ap: mp_ptr,
                                       mp: mp_srcptr,
                                       n: size_t,
                                       nbcnt: bitcnt_t,
                                       tp: mp_ptr)
                                       -> c_int;
    "__gmpn_sec_invert_itch" mpn_sec_invert_itch(n: size_t) -> size_t;
}

// Random Numbers

c_fn! {
    // Random State Initialization
    "__gmp_randinit_default" randinit_default(state: randstate_ptr);
    "__gmp_randinit_mt" randinit_mt(state: randstate_ptr);
    "__gmp_randinit_lc_2exp" randinit_lc_2exp(state: randstate_ptr,
                                              a: mpz_srcptr,
                                              c: c_ulong,
                                              m2exp: bitcnt_t);
    "__gmp_randinit_lc_2exp_size" randinit_lc_2exp_size(state: randstate_ptr,
                                                        size: bitcnt_t)
                                                        -> c_int;
    "__gmp_randinit_set" randinit_set(rop: randstate_ptr, op: randstate_srcptr);
    "__gmp_randclear" randclear(state: randstate_ptr);

    // Random State Seeding
    "__gmp_randseed" randseed(state: randstate_ptr, seed: mpz_srcptr);
    "__gmp_randseed_ui" randseed_ui(state: randstate_ptr, seed: c_ulong);

    // Random State Miscellaneous
    "__gmp_urandomb_ui" urandomb_ui(state: randstate_ptr,
                                    n: c_ulong)
                                    -> c_ulong;
    "__gmp_urandomm_ui" urandomm_ui(state: randstate_ptr,
                                    n: c_ulong)
                                    -> c_ulong;
}

// Formatted Output
c_fn! {
    "__gmp_printf" printf(fmt: *const c_char; ...) -> c_int;
    "__gmp_sprintf" sprintf(buf: *mut c_char, fmt: *const c_char; ...) -> c_int;
    "__gmp_snprintf" snprintf(buf: *mut c_char,
                              size: usize,
                              fmt: *const c_char;
                              ...)
                              -> c_int;
    "__gmp_asprintf" asprintf(pp: *mut *mut c_char,
                              fmt: *const c_char;
                              ...)
                              -> c_int;
}

// Formatted Input
c_fn! {
    "__gmp_scanf" scanf(fmt: *const c_char; ...) -> c_int;
    "__gmp_sscanf" sscanf(s: *const c_char, fmt: *const c_char; ...) -> c_int;
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
    "__gmp_set_memory_functions" set_memory_functions(
        alloc_func_ptr: allocate_function,
        realloc_func_ptr: reallocate_function,
        free_func_ptr: free_function
    );
    "__gmp_get_memory_functions" get_memory_functions(
        alloc_func_ptr: *mut allocate_function,
        realloc_func_ptr: *mut reallocate_function,
        free_func_ptr: *mut free_function
    );
}
