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

extern "C" {
    #[link_name = "__gmp_bits_per_limb"]
    pub static bits_per_limb: c_int;
}
pub const VERSION: c_int = 6;
pub const VERSION_MINOR: c_int = 1;
pub const VERSION_PATCHLEVEL: c_int = 2;
extern "C" {
    #[link_name = "__gmp_version"]
    pub static version: *const c_char;
}

pub type exp_t = c_long;
include!(concat!(env!("OUT_DIR"), "/mp_limb_t.rs"));
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

extern "C" {
    // Initialization Functions

    #[link_name = "__gmpz_init"]
    pub fn mpz_init(x: mpz_ptr);
    #[link_name = "__gmpz_inits"]
    pub fn mpz_inits(x: mpz_ptr, ...);
    #[link_name = "__gmpz_init2"]
    pub fn mpz_init2(x: mpz_ptr, n: bitcnt_t);
    #[link_name = "__gmpz_clear"]
    pub fn mpz_clear(x: mpz_ptr);
    #[link_name = "__gmpz_clears"]
    pub fn mpz_clears(x: mpz_ptr, ...);
    #[link_name = "__gmpz_realloc2"]
    pub fn mpz_realloc2(x: mpz_ptr, n: bitcnt_t);

    // Assignment Functions

    #[link_name = "__gmpz_set"]
    pub fn mpz_set(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_set_ui"]
    pub fn mpz_set_ui(rop: mpz_ptr, op: c_ulong);
    #[link_name = "__gmpz_set_si"]
    pub fn mpz_set_si(rop: mpz_ptr, op: c_long);
    #[link_name = "__gmpz_set_d"]
    pub fn mpz_set_d(rop: mpz_ptr, op: f64);
    #[link_name = "__gmpz_set_q"]
    pub fn mpz_set_q(rop: mpz_ptr, op: mpq_srcptr);
    #[link_name = "__gmpz_set_f"]
    pub fn mpz_set_f(rop: mpz_ptr, op: mpf_srcptr);
    #[link_name = "__gmpz_set_str"]
    pub fn mpz_set_str(rop: mpz_ptr, str: *const c_char, base: c_int) -> c_int;
    #[link_name = "__gmpz_swap"]
    pub fn mpz_swap(rop1: mpz_ptr, rop2: mpz_ptr);

    // Combined Initialization and Assignment Functions

    #[link_name = "__gmpz_init_set"]
    pub fn mpz_init_set(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_init_set_ui"]
    pub fn mpz_init_set_ui(rop: mpz_ptr, op: c_ulong);
    #[link_name = "__gmpz_init_set_si"]
    pub fn mpz_init_set_si(rop: mpz_ptr, op: c_long);
    #[link_name = "__gmpz_init_set_d"]
    pub fn mpz_init_set_d(rop: mpz_ptr, op: f64);
    #[link_name = "__gmpz_init_set_str"]
    pub fn mpz_init_set_str(rop: mpz_ptr,
                            str: *const c_char,
                            base: c_int)
                            -> c_int;

    // Conversion Functions

    #[link_name = "__gmpz_get_ui"]
    pub fn mpz_get_ui(op: mpz_srcptr) -> c_ulong;
    #[link_name = "__gmpz_get_si"]
    pub fn mpz_get_si(op: mpz_srcptr) -> c_long;
    #[link_name = "__gmpz_get_d"]
    pub fn mpz_get_d(op: mpz_srcptr) -> f64;
    #[link_name = "__gmpz_get_d_2exp"]
    pub fn mpz_get_d_2exp(exp: *mut c_long, op: mpz_srcptr) -> f64;
    #[link_name = "__gmpz_get_str"]
    pub fn mpz_get_str(str: *mut c_char,
                       base: c_int,
                       op: mpz_srcptr)
                       -> *mut c_char;

    // Arithmetic Functions

    #[link_name = "__gmpz_add"]
    pub fn mpz_add(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_add_ui"]
    pub fn mpz_add_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_sub"]
    pub fn mpz_sub(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_sub_ui"]
    pub fn mpz_sub_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_ui_sub"]
    pub fn mpz_ui_sub(rop: mpz_ptr, op1: c_ulong, op2: mpz_srcptr);
    #[link_name = "__gmpz_mul"]
    pub fn mpz_mul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_mul_si"]
    pub fn mpz_mul_si(rop: mpz_ptr, op1: mpz_srcptr, op2: c_long);
    #[link_name = "__gmpz_mul_ui"]
    pub fn mpz_mul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_addmul"]
    pub fn mpz_addmul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_addmul_ui"]
    pub fn mpz_addmul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_submul"]
    pub fn mpz_submul(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_submul_ui"]
    pub fn mpz_submul_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_mul_2exp"]
    pub fn mpz_mul_2exp(rop: mpz_ptr, op1: mpz_srcptr, op2: bitcnt_t);
    #[link_name = "__gmpz_neg"]
    pub fn mpz_neg(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_abs"]
    pub fn mpz_abs(rop: mpz_ptr, op: mpz_srcptr);

    // Division Functions

    #[link_name = "__gmpz_cdiv_q"]
    pub fn mpz_cdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_cdiv_r"]
    pub fn mpz_cdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_cdiv_qr"]
    pub fn mpz_cdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_cdiv_q_ui"]
    pub fn mpz_cdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_cdiv_r_ui"]
    pub fn mpz_cdiv_r_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_cdiv_qr_ui"]
    pub fn mpz_cdiv_qr_ui(q: mpz_ptr,
                          r: mpz_ptr,
                          n: mpz_srcptr,
                          d: c_ulong)
                          -> c_ulong;
    #[link_name = "__gmpz_cdiv_ui"]
    pub fn mpz_cdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_cdiv_q_2exp"]
    pub fn mpz_cdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_cdiv_r_2exp"]
    pub fn mpz_cdiv_r_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_fdiv_q"]
    pub fn mpz_fdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_fdiv_r"]
    pub fn mpz_fdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_fdiv_qr"]
    pub fn mpz_fdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_fdiv_q_ui"]
    pub fn mpz_fdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_fdiv_r_ui"]
    pub fn mpz_fdiv_r_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_fdiv_qr_ui"]
    pub fn mpz_fdiv_qr_ui(q: mpz_ptr,
                          r: mpz_ptr,
                          n: mpz_srcptr,
                          d: c_ulong)
                          -> c_ulong;
    #[link_name = "__gmpz_fdiv_ui"]
    pub fn mpz_fdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_fdiv_q_2exp"]
    pub fn mpz_fdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_fdiv_r_2exp"]
    pub fn mpz_fdiv_r_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_tdiv_q"]
    pub fn mpz_tdiv_q(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_tdiv_r"]
    pub fn mpz_tdiv_r(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_tdiv_qr"]
    pub fn mpz_tdiv_qr(q: mpz_ptr, r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_tdiv_q_ui"]
    pub fn mpz_tdiv_q_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_tdiv_r_ui"]
    pub fn mpz_tdiv_r_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_tdiv_qr_ui"]
    pub fn mpz_tdiv_qr_ui(q: mpz_ptr,
                          r: mpz_ptr,
                          n: mpz_srcptr,
                          d: c_ulong)
                          -> c_ulong;
    #[link_name = "__gmpz_tdiv_ui"]
    pub fn mpz_tdiv_ui(n: mpz_srcptr, d: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_tdiv_q_2exp"]
    pub fn mpz_tdiv_q_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_tdiv_r_2exp"]
    pub fn mpz_tdiv_r_2exp(q: mpz_ptr, n: mpz_srcptr, b: bitcnt_t);
    #[link_name = "__gmpz_mod"]
    pub fn mpz_mod(r: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
}
#[inline]
pub unsafe fn mpz_mod_ui(r: mpz_ptr, n: mpz_srcptr, d: c_ulong) -> c_ulong {
    mpz_fdiv_r_ui(r, n, d)
}
extern "C" {
    #[link_name = "__gmpz_divexact"]
    pub fn mpz_divexact(q: mpz_ptr, n: mpz_srcptr, d: mpz_srcptr);
    #[link_name = "__gmpz_divexact_ui"]
    pub fn mpz_divexact_ui(q: mpz_ptr, n: mpz_srcptr, d: c_ulong);
    #[link_name = "__gmpz_divisible_p"]
    pub fn mpz_divisible_p(n: mpz_srcptr, d: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_divisible_ui_p"]
    pub fn mpz_divisible_ui_p(n: mpz_srcptr, d: c_ulong) -> c_int;
    #[link_name = "__gmpz_divisible_2exp_p"]
    pub fn mpz_divisible_2exp_p(n: mpz_srcptr, b: bitcnt_t) -> c_int;
    #[link_name = "__gmpz_congruent_p"]
    pub fn mpz_congruent_p(n: mpz_srcptr,
                           c: mpz_srcptr,
                           d: mpz_srcptr)
                           -> c_int;
    #[link_name = "__gmpz_congruent_ui_p"]
    pub fn mpz_congruent_ui_p(n: mpz_srcptr, c: c_ulong, d: c_ulong) -> c_int;
    #[link_name = "__gmpz_congruent_2exp_p"]
    pub fn mpz_congruent_2exp_p(n: mpz_srcptr,
                                c: mpz_srcptr,
                                b: bitcnt_t)
                                -> c_int;

    // Exponentiation Functions

    #[link_name = "__gmpz_powm"]
    pub fn mpz_powm(rop: mpz_ptr,
                    base: mpz_srcptr,
                    exp: mpz_srcptr,
                    modu: mpz_srcptr);
    #[link_name = "__gmpz_powm_ui"]
    pub fn mpz_powm_ui(rop: mpz_ptr,
                       base: mpz_srcptr,
                       exp: c_ulong,
                       modu: mpz_srcptr);
    #[link_name = "__gmpz_powm_sec"]
    pub fn mpz_powm_sec(rop: mpz_ptr,
                        base: mpz_srcptr,
                        exp: mpz_srcptr,
                        modu: mpz_srcptr);
    #[link_name = "__gmpz_pow_ui"]
    pub fn mpz_pow_ui(rop: mpz_ptr, base: mpz_srcptr, exp: c_ulong);
    #[link_name = "__gmpz_ui_pow_ui"]
    pub fn mpz_ui_pow_ui(rop: mpz_ptr, base: c_ulong, exp: c_ulong);

    // Root Extraction Functions

    #[link_name = "__gmpz_root"]
    pub fn mpz_root(rop: mpz_ptr, op: mpz_srcptr, n: c_ulong) -> c_int;
    #[link_name = "__gmpz_rootrem"]
    pub fn mpz_rootrem(root: mpz_ptr,
                       rem: mpz_ptr,
                       op: mpz_srcptr,
                       n: c_ulong);
    #[link_name = "__gmpz_sqrt"]
    pub fn mpz_sqrt(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_sqrtrem"]
    pub fn mpz_sqrtrem(rop1: mpz_ptr, rop2: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_perfect_power_p"]
    pub fn mpz_perfect_power_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_perfect_square_p"]
    pub fn mpz_perfect_square_p(op: mpz_srcptr) -> c_int;

    // Number Theoretic Functions

    #[link_name = "__gmpz_probab_prime_p"]
    pub fn mpz_probab_prime_p(n: mpz_srcptr, reps: c_int) -> c_int;
    #[link_name = "__gmpz_nextprime"]
    pub fn mpz_nextprime(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_gcd"]
    pub fn mpz_gcd(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_gcd_ui"]
    pub fn mpz_gcd_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong) -> c_ulong;
    #[link_name = "__gmpz_gcdext"]
    pub fn mpz_gcdext(g: mpz_ptr,
                      s: mpz_ptr,
                      t: mpz_ptr,
                      a: mpz_srcptr,
                      b: mpz_srcptr);
    #[link_name = "__gmpz_lcm"]
    pub fn mpz_lcm(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_lcm_ui"]
    pub fn mpz_lcm_ui(rop: mpz_ptr, op1: mpz_srcptr, op2: c_ulong);
    #[link_name = "__gmpz_invert"]
    pub fn mpz_invert(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_jacobi"]
    pub fn mpz_jacobi(a: mpz_srcptr, b: mpz_srcptr) -> c_int;
}
#[inline]
pub unsafe fn mpz_legendre(a: mpz_srcptr, p: mpz_srcptr) -> c_int {
    mpz_jacobi(a, p)
}
#[inline]
pub unsafe fn mpz_kronecker(a: mpz_srcptr, b: mpz_srcptr) -> c_int {
    mpz_jacobi(a, b)
}
extern "C" {
    #[link_name = "__gmpz_kronecker_si"]
    pub fn mpz_kronecker_si(a: mpz_srcptr, b: c_long) -> c_int;
    #[link_name = "__gmpz_kronecker_ui"]
    pub fn mpz_kronecker_ui(a: mpz_srcptr, b: c_ulong) -> c_int;
    #[link_name = "__gmpz_si_kronecker"]
    pub fn mpz_si_kronecker(a: c_long, b: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_ui_kronecker"]
    pub fn mpz_ui_kronecker(a: c_ulong, b: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_remove"]
    pub fn mpz_remove(rop: mpz_ptr, op: mpz_srcptr, f: mpz_srcptr) -> bitcnt_t;
    #[link_name = "__gmpz_fac_ui"]
    pub fn mpz_fac_ui(rop: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_2fac_ui"]
    pub fn mpz_2fac_ui(rop: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_mfac_uiui"]
    pub fn mpz_mfac_uiui(rop: mpz_ptr, n: c_ulong, m: c_ulong);
    #[link_name = "__gmpz_primorial_ui"]
    pub fn mpz_primorial_ui(r: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_bin_ui"]
    pub fn mpz_bin_ui(rop: mpz_ptr, n: mpz_srcptr, k: c_ulong);
    #[link_name = "__gmpz_bin_uiui"]
    pub fn mpz_bin_uiui(rop: mpz_ptr, n: c_ulong, k: c_ulong);
    #[link_name = "__gmpz_fib_ui"]
    pub fn mpz_fib_ui(f_n: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_fib2_ui"]
    pub fn mpz_fib2_ui(f_n: mpz_ptr, fnsub1: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_lucnum_ui"]
    pub fn mpz_lucnum_ui(ln: mpz_ptr, n: c_ulong);
    #[link_name = "__gmpz_lucnum2_ui"]
    pub fn mpz_lucnum2_ui(ln: mpz_ptr, lnsub1: mpz_ptr, n: c_ulong);

    // Comparison Functions

    #[link_name = "__gmpz_cmp"]
    pub fn mpz_cmp(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_cmp_d"]
    pub fn mpz_cmp_d(op1: mpz_srcptr, op2: f64) -> c_int;
    #[link_name = "__gmpz_cmp_si"]
    pub fn mpz_cmp_si(op1: mpz_srcptr, op2: c_long) -> c_int;
    #[link_name = "__gmpz_cmp_ui"]
    pub fn mpz_cmp_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
    #[link_name = "__gmpz_cmpabs"]
    pub fn mpz_cmpabs(op1: mpz_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_cmpabs_d"]
    pub fn mpz_cmpabs_d(op1: mpz_srcptr, op2: f64) -> c_int;
    #[link_name = "__gmpz_cmpabs_ui"]
    pub fn mpz_cmpabs_ui(op1: mpz_srcptr, op2: c_ulong) -> c_int;
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
extern "C" {
    #[link_name = "__gmpz_and"]
    pub fn mpz_and(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_ior"]
    pub fn mpz_ior(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_xor"]
    pub fn mpz_xor(rop: mpz_ptr, op1: mpz_srcptr, op2: mpz_srcptr);
    #[link_name = "__gmpz_com"]
    pub fn mpz_com(rop: mpz_ptr, op: mpz_srcptr);
    #[link_name = "__gmpz_popcount"]
    pub fn mpz_popcount(op: mpz_srcptr) -> bitcnt_t;
    #[link_name = "__gmpz_hamdist"]
    pub fn mpz_hamdist(op1: mpz_srcptr, op2: mpz_srcptr) -> bitcnt_t;
    #[link_name = "__gmpz_scan0"]
    pub fn mpz_scan0(op: mpz_srcptr, starting_bit: bitcnt_t) -> bitcnt_t;
    #[link_name = "__gmpz_scan1"]
    pub fn mpz_scan1(op: mpz_srcptr, starting_bit: bitcnt_t) -> bitcnt_t;
    #[link_name = "__gmpz_setbit"]
    pub fn mpz_setbit(rop: mpz_ptr, bit_index: bitcnt_t);
    #[link_name = "__gmpz_clrbit"]
    pub fn mpz_clrbit(rop: mpz_ptr, bit_index: bitcnt_t);
    #[link_name = "__gmpz_combit"]
    pub fn mpz_combit(rop: mpz_ptr, bit_index: bitcnt_t);
    #[link_name = "__gmpz_tstbit"]
    pub fn mpz_tstbit(rop: mpz_srcptr, bit_index: bitcnt_t) -> c_int;

    // Random Number Functions

    #[link_name = "__gmpz_urandomb"]
    pub fn mpz_urandomb(rop: mpz_ptr, state: randstate_ptr, n: bitcnt_t);
    #[link_name = "__gmpz_urandomm"]
    pub fn mpz_urandomm(rop: mpz_ptr, state: randstate_ptr, n: mpz_srcptr);
    #[link_name = "__gmpz_rrandomb"]
    pub fn mpz_rrandomb(rop: mpz_ptr, state: randstate_ptr, n: bitcnt_t);
    #[link_name = "__gmpz_random2"]
    pub fn mpz_random2(rop: mpz_ptr, max_size: size_t);

    // Integer Import and Export

    #[link_name = "__gmpz_import"]
    pub fn mpz_import(rop: mpz_ptr,
                      count: usize,
                      order: c_int,
                      size: usize,
                      endian: c_int,
                      nails: usize,
                      op: *const c_void);
    #[link_name = "__gmpz_export"]
    pub fn mpz_export(rop: *mut c_void,
                      countp: *mut usize,
                      order: c_int,
                      size: usize,
                      endian: c_int,
                      nails: usize,
                      op: mpz_srcptr)
                      -> *mut c_void;

    // Miscellaneous Functions

    #[link_name = "__gmpz_fits_ulong_p"]
    pub fn mpz_fits_ulong_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_fits_slong_p"]
    pub fn mpz_fits_slong_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_fits_uint_p"]
    pub fn mpz_fits_uint_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_fits_sint_p"]
    pub fn mpz_fits_sint_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_fits_ushort_p"]
    pub fn mpz_fits_ushort_p(op: mpz_srcptr) -> c_int;
    #[link_name = "__gmpz_fits_sshort_p"]
    pub fn mpz_fits_sshort_p(op: mpz_srcptr) -> c_int;
}
#[inline]
pub unsafe fn mpz_odd_p(op: mpz_srcptr) -> c_int {
    (*(*op).d) as c_int & if (*op).size != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn mpz_even_p(op: mpz_srcptr) -> c_int {
    !mpz_odd_p(op)
}
extern "C" {
    #[link_name = "__gmpz_sizeinbase"]
    pub fn mpz_sizeinbase(arg1: mpz_srcptr, arg2: c_int) -> usize;

    // Special Functions

    #[link_name = "__gmpz_realloc"]
    pub fn _mpz_realloc(integer: mpz_ptr, new_alloc: size_t) -> *mut c_void;
    #[link_name = "__gmpz_getlimbn"]
    pub fn mpz_getlimbn(op: mpz_srcptr, n: size_t) -> limb_t;
    #[link_name = "__gmpz_size"]
    pub fn mpz_size(op: mpz_srcptr) -> usize;
    #[link_name = "__gmpz_limbs_read"]
    pub fn mpz_limbs_read(x: mpz_srcptr) -> mp_srcptr;
    #[link_name = "__gmpz_limbs_write"]
    pub fn mpz_limbs_write(x: mpz_ptr, n: size_t) -> mp_ptr;
    #[link_name = "__gmpz_limbs_modify"]
    pub fn mpz_limbs_modify(x: mpz_ptr, n: size_t) -> mp_ptr;
    #[link_name = "__gmpz_limbs_finish"]
    pub fn mpz_limbs_finish(x: mpz_ptr, s: size_t);
    #[link_name = "__gmpz_roinit_n"]
    pub fn mpz_roinit_n(x: mpz_ptr, xp: mp_srcptr, xs: size_t) -> mpz_srcptr;
}

// Rational numbers

extern "C" {
    #[link_name = "__gmpq_canonicalize"]
    pub fn mpq_canonicalize(op: mpq_ptr);

    // Initialization and Assignment Functions

    #[link_name = "__gmpq_init"]
    pub fn mpq_init(x: mpq_ptr);
    #[link_name = "__gmpq_inits"]
    pub fn mpq_inits(x: mpq_ptr, ...);
    #[link_name = "__gmpq_clear"]
    pub fn mpq_clear(x: mpq_ptr);
    #[link_name = "__gmpq_clears"]
    pub fn mpq_clears(x: mpq_ptr, ...);
    #[link_name = "__gmpq_set"]
    pub fn mpq_set(rop: mpq_ptr, op: mpq_srcptr);
    #[link_name = "__gmpq_set_z"]
    pub fn mpq_set_z(rop: mpq_ptr, op: mpz_srcptr);
    #[link_name = "__gmpq_set_ui"]
    pub fn mpq_set_ui(rop: mpq_ptr, op1: c_ulong, op2: c_ulong);
    #[link_name = "__gmpq_set_si"]
    pub fn mpq_set_si(rop: mpq_ptr, op1: c_long, op2: c_ulong);
    #[link_name = "__gmpq_set_str"]
    pub fn mpq_set_str(rop: mpq_ptr, str: *const c_char, base: c_int) -> c_int;
    #[link_name = "__gmpq_swap"]
    pub fn mpq_swap(rop1: mpq_ptr, rop2: mpq_ptr);

    // Conversion Functions

    #[link_name = "__gmpq_get_d"]
    pub fn mpq_get_d(op: mpq_srcptr) -> f64;
    #[link_name = "__gmpq_set_d"]
    pub fn mpq_set_d(rop: mpq_ptr, op: f64);
    #[link_name = "__gmpq_set_f"]
    pub fn mpq_set_f(rop: mpq_ptr, op: mpf_srcptr);
    #[link_name = "__gmpq_get_str"]
    pub fn mpq_get_str(str: *mut c_char,
                       base: c_int,
                       op: mpq_srcptr)
                       -> *mut c_char;

    // Arithmetic Functions

    #[link_name = "__gmpq_add"]
    pub fn mpq_add(sum: mpq_ptr, addend1: mpq_srcptr, addend2: mpq_srcptr);
    #[link_name = "__gmpq_sub"]
    pub fn mpq_sub(difference: mpq_ptr,
                   minuend: mpq_srcptr,
                   subtrahend: mpq_srcptr);
    #[link_name = "__gmpq_mul"]
    pub fn mpq_mul(product: mpq_ptr,
                   multiplier: mpq_srcptr,
                   multiplicand: mpq_srcptr);
    #[link_name = "__gmpq_mul_2exp"]
    pub fn mpq_mul_2exp(rop: mpq_ptr, op1: mpq_srcptr, op2: bitcnt_t);
    #[link_name = "__gmpq_div"]
    pub fn mpq_div(quotient: mpq_ptr,
                   dividend: mpq_srcptr,
                   divisor: mpq_srcptr);
    #[link_name = "__gmpq_div_2exp"]
    pub fn mpq_div_2exp(rop: mpq_ptr, op1: mpq_srcptr, op2: bitcnt_t);
    #[link_name = "__gmpq_neg"]
    pub fn mpq_neg(negated_operand: mpq_ptr, operand: mpq_srcptr);
    #[link_name = "__gmpq_abs"]
    pub fn mpq_abs(rop: mpq_ptr, op: mpq_srcptr);
    #[link_name = "__gmpq_inv"]
    pub fn mpq_inv(inverted_number: mpq_ptr, number: mpq_srcptr);

    // Comparison Functions

    #[link_name = "__gmpq_cmp"]
    pub fn mpq_cmp(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
    #[link_name = "__gmpq_cmp_z"]
    pub fn mpq_cmp_z(op1: mpq_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "__gmpq_cmp_ui"]
    pub fn mpq_cmp_ui(op1: mpq_srcptr, num2: c_ulong, den2: c_ulong) -> c_int;
    #[link_name = "__gmpq_cmp_si"]
    pub fn mpq_cmp_si(op1: mpq_srcptr, num2: c_long, den2: c_ulong) -> c_int;
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
extern "C" {
    #[link_name = "__gmpq_equal"]
    pub fn mpq_equal(op1: mpq_srcptr, op2: mpq_srcptr) -> c_int;
}

// Applying Integer Functions to Rationals

#[inline]
pub unsafe fn mpq_numref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op).num) as mpz_ptr
}
#[inline]
pub unsafe fn mpq_denref(op: mpq_ptr) -> mpz_ptr {
    (&mut (*op).den) as mpz_ptr
}
extern "C" {
    #[link_name = "__gmpq_get_num"]
    pub fn mpq_get_num(numerator: mpz_ptr, rational: mpq_srcptr);
    #[link_name = "__gmpq_get_den"]
    pub fn mpq_get_den(denominator: mpz_ptr, rational: mpq_srcptr);
    #[link_name = "__gmpq_set_den"]
    pub fn mpq_set_den(rational: mpq_ptr, numerator: mpz_srcptr);
    #[link_name = "__gmpq_set_num"]
    pub fn mpq_set_num(rational: mpq_ptr, denominator: mpz_srcptr);
}

// Floating-point numbers

extern "C" {
    // Initialization Functions

    #[link_name = "__gmpf_set_default_prec"]
    pub fn mpf_set_default_prec(prec: bitcnt_t);
    #[link_name = "__gmpf_get_default_prec"]
    pub fn mpf_get_default_prec() -> bitcnt_t;
    #[link_name = "__gmpf_init"]
    pub fn mpf_init(x: mpf_ptr);
    #[link_name = "__gmpf_init2"]
    pub fn mpf_init2(x: mpf_ptr, prec: bitcnt_t);
    #[link_name = "__gmpf_inits"]
    pub fn mpf_inits(x: mpf_ptr, ...);
    #[link_name = "__gmpf_clear"]
    pub fn mpf_clear(x: mpf_ptr);
    #[link_name = "__gmpf_clears"]
    pub fn mpf_clears(x: mpf_ptr, ...);
    #[link_name = "__gmpf_get_prec"]
    pub fn mpf_get_prec(op: mpf_srcptr) -> bitcnt_t;
    #[link_name = "__gmpf_set_prec"]
    pub fn mpf_set_prec(rop: mpf_ptr, prec: bitcnt_t);
    #[link_name = "__gmpf_set_prec_raw"]
    pub fn mpf_set_prec_raw(rop: mpf_ptr, prec: bitcnt_t);

    // Assignment Functions

    #[link_name = "__gmpf_set"]
    pub fn mpf_set(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_set_ui"]
    pub fn mpf_set_ui(rop: mpf_ptr, op: c_ulong);
    #[link_name = "__gmpf_set_si"]
    pub fn mpf_set_si(rop: mpf_ptr, op: c_long);
    #[link_name = "__gmpf_set_d"]
    pub fn mpf_set_d(rop: mpf_ptr, op: f64);
    #[link_name = "__gmpf_set_z"]
    pub fn mpf_set_z(rop: mpf_ptr, op: mpz_srcptr);
    #[link_name = "__gmpf_set_q"]
    pub fn mpf_set_q(rop: mpf_ptr, op: mpq_srcptr);
    #[link_name = "__gmpf_set_str"]
    pub fn mpf_set_str(rop: mpf_ptr, str: *const c_char, base: c_int) -> c_int;
    #[link_name = "__gmpf_swap"]
    pub fn mpf_swap(rop1: mpf_ptr, rop2: mpf_ptr);

    // Combined Initialization and Assignment Functions

    #[link_name = "__gmpf_init_set"]
    pub fn mpf_init_set(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_init_set_ui"]
    pub fn mpf_init_set_ui(rop: mpf_ptr, op: c_ulong);
    #[link_name = "__gmpf_init_set_si"]
    pub fn mpf_init_set_si(rop: mpf_ptr, op: c_long);
    #[link_name = "__gmpf_init_set_d"]
    pub fn mpf_init_set_d(rop: mpf_ptr, op: f64);
    #[link_name = "__gmpf_init_set_str"]
    pub fn mpf_init_set_str(rop: mpf_ptr,
                            str: *const c_char,
                            base: c_int)
                            -> c_int;

    // Conversion Functions

    #[link_name = "__gmpf_get_d"]
    pub fn mpf_get_d(op: mpf_srcptr) -> f64;
    #[link_name = "__gmpf_get_d_2exp"]
    pub fn mpf_get_d_2exp(exp: *mut c_long, op: mpf_srcptr) -> f64;
    #[link_name = "__gmpf_get_si"]
    pub fn mpf_get_si(op: mpf_srcptr) -> c_long;
    #[link_name = "__gmpf_get_ui"]
    pub fn mpf_get_ui(op: mpf_srcptr) -> c_ulong;
    #[link_name = "__gmpf_get_str"]
    pub fn mpf_get_str(str: *mut c_char,
                       expptr: *mut exp_t,
                       base: c_int,
                       n_digits: usize,
                       op: mpf_srcptr)
                       -> *mut c_char;

    // Arithmetic Functions

    #[link_name = "__gmpf_add"]
    pub fn mpf_add(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    #[link_name = "__gmpf_add_ui"]
    pub fn mpf_add_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    #[link_name = "__gmpf_sub"]
    pub fn mpf_sub(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    #[link_name = "__gmpf_ui_sub"]
    pub fn mpf_ui_sub(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    #[link_name = "__gmpf_sub_ui"]
    pub fn mpf_sub_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    #[link_name = "__gmpf_mul"]
    pub fn mpf_mul(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    #[link_name = "__gmpf_mul_ui"]
    pub fn mpf_mul_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    #[link_name = "__gmpf_div"]
    pub fn mpf_div(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
    #[link_name = "__gmpf_ui_div"]
    pub fn mpf_ui_div(rop: mpf_ptr, op1: c_ulong, op2: mpf_srcptr);
    #[link_name = "__gmpf_div_ui"]
    pub fn mpf_div_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    #[link_name = "__gmpf_sqrt"]
    pub fn mpf_sqrt(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_sqrt_ui"]
    pub fn mpf_sqrt_ui(rop: mpf_ptr, op: c_ulong);
    #[link_name = "__gmpf_pow_ui"]
    pub fn mpf_pow_ui(rop: mpf_ptr, op1: mpf_srcptr, op2: c_ulong);
    #[link_name = "__gmpf_neg"]
    pub fn mpf_neg(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_abs"]
    pub fn mpf_abs(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_mul_2exp"]
    pub fn mpf_mul_2exp(rop: mpf_ptr, op1: mpf_srcptr, op2: bitcnt_t);
    #[link_name = "__gmpf_div_2exp"]
    pub fn mpf_div_2exp(rop: mpf_ptr, op1: mpf_srcptr, op2: bitcnt_t);

    // Comparison Functions

    #[link_name = "__gmpf_cmp"]
    pub fn mpf_cmp(op1: mpf_srcptr, op2: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_cmp_z"]
    pub fn mpf_cmp_z(op1: mpf_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "__gmpf_cmp_d"]
    pub fn mpf_cmp_d(op1: mpf_srcptr, op2: f64) -> c_int;
    #[link_name = "__gmpf_cmp_ui"]
    pub fn mpf_cmp_ui(op1: mpf_srcptr, op2: c_ulong) -> c_int;
    #[link_name = "__gmpf_cmp_si"]
    pub fn mpf_cmp_si(op1: mpf_srcptr, op2: c_long) -> c_int;
    #[link_name = "__gmpf_eq"]
    pub fn mpf_eq(op1: mpf_srcptr, op2: mpf_srcptr, op3: bitcnt_t) -> c_int;
    #[link_name = "__gmpf_reldiff"]
    pub fn mpf_reldiff(rop: mpf_ptr, op1: mpf_srcptr, op2: mpf_srcptr);
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
extern "C" {
    // Miscellaneous Functions

    #[link_name = "__gmpf_ceil"]
    pub fn mpf_ceil(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_floor"]
    pub fn mpf_floor(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_trunc"]
    pub fn mpf_trunc(rop: mpf_ptr, op: mpf_srcptr);
    #[link_name = "__gmpf_integer_p"]
    pub fn mpf_integer_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_ulong_p"]
    pub fn mpf_fits_ulong_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_slong_p"]
    pub fn mpf_fits_slong_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_uint_p"]
    pub fn mpf_fits_uint_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_sint_p"]
    pub fn mpf_fits_sint_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_ushort_p"]
    pub fn mpf_fits_ushort_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_fits_sshort_p"]
    pub fn mpf_fits_sshort_p(op: mpf_srcptr) -> c_int;
    #[link_name = "__gmpf_urandomb"]
    pub fn mpf_urandomb(rop: mpf_t, state: randstate_ptr, nbits: bitcnt_t);
    #[link_name = "__gmpf_random2"]
    pub fn mpf_random2(rop: mpf_ptr, max_size: size_t, exp: exp_t);
}

// Low-Level Functions

extern "C" {
    #[link_name = "__gmpn_add_n"]
    pub fn mpn_add_n(rp: mp_ptr,
                     s1p: mp_srcptr,
                     s2p: mp_srcptr,
                     n: size_t)
                     -> limb_t;
    #[link_name = "__gmpn_add_1"]
    pub fn mpn_add_1(rp: mp_ptr,
                     s1p: mp_srcptr,
                     n: size_t,
                     s2limb: limb_t)
                     -> limb_t;
    #[link_name = "__gmpn_add"]
    pub fn mpn_add(rp: mp_ptr,
                   s1p: mp_srcptr,
                   s1n: size_t,
                   s2p: mp_srcptr,
                   s2n: size_t)
                   -> limb_t;
    #[link_name = "__gmpn_sub_n"]
    pub fn mpn_sub_n(rp: mp_ptr,
                     s1p: mp_srcptr,
                     s2p: mp_srcptr,
                     n: size_t)
                     -> limb_t;
    #[link_name = "__gmpn_sub_1"]
    pub fn mpn_sub_1(rp: mp_ptr,
                     s1p: mp_srcptr,
                     n: size_t,
                     s2limb: limb_t)
                     -> limb_t;
    #[link_name = "__gmpn_sub"]
    pub fn mpn_sub(rp: mp_ptr,
                   s1p: mp_srcptr,
                   s1n: size_t,
                   s2p: mp_srcptr,
                   s2n: size_t)
                   -> limb_t;
    #[link_name = "__gmpn_neg"]
    pub fn mpn_neg(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t;
    #[link_name = "__gmpn_mul_n"]
    pub fn mpn_mul_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_mul"]
    pub fn mpn_mul(rp: mp_ptr,
                   s1p: mp_srcptr,
                   s1n: size_t,
                   s2p: mp_srcptr,
                   s2n: size_t)
                   -> limb_t;
    #[link_name = "__gmpn_sqr"]
    pub fn mpn_sqr(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_mul_1"]
    pub fn mpn_mul_1(rp: mp_ptr,
                     s1p: mp_srcptr,
                     n: size_t,
                     s2limb: limb_t)
                     -> limb_t;
    #[link_name = "__gmpn_addmul_1"]
    pub fn mpn_addmul_1(rp: mp_ptr,
                        s1p: mp_srcptr,
                        n: size_t,
                        s2limb: limb_t)
                        -> limb_t;
    #[link_name = "__gmpn_submul_1"]
    pub fn mpn_submul_1(rp: mp_ptr,
                        s1p: mp_srcptr,
                        n: size_t,
                        s2limb: limb_t)
                        -> limb_t;
    #[link_name = "__gmpn_tdiv_qr"]
    pub fn mpn_tdiv_qr(qp: mp_ptr,
                       rp: mp_ptr,
                       qxn: size_t,
                       np: mp_srcptr,
                       nn: size_t,
                       dp: mp_srcptr,
                       dn: size_t);
    #[link_name = "__gmpn_divrem_1"]
    pub fn mpn_divrem_1(r1p: mp_ptr,
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
extern "C" {
    #[link_name = "__gmpn_divexact_1"]
    pub fn mpn_divexact_1(rp: mp_ptr, sp: mp_srcptr, n: size_t, d: limb_t);
}
#[inline]
pub unsafe fn mpn_divexact_by3(rp: mp_ptr, sp: mp_srcptr, n: size_t) -> limb_t {
    mpn_divexact_by3c(rp, sp, n, 0)
}
extern "C" {
    #[link_name = "__gmpn_divexact_by3c"]
    pub fn mpn_divexact_by3c(rp: mp_ptr,
                             sp: mp_srcptr,
                             n: size_t,
                             carry: limb_t)
                             -> limb_t;
    #[link_name = "__gmpn_mod_1"]
    pub fn mpn_mod_1(s1p: mp_srcptr, s1n: size_t, s2limb: limb_t) -> limb_t;
    #[link_name = "__gmpn_lshift"]
    pub fn mpn_lshift(rp: mp_ptr,
                      sp: mp_srcptr,
                      n: size_t,
                      count: c_uint)
                      -> limb_t;
    #[link_name = "__gmpn_rshift"]
    pub fn mpn_rshift(rp: mp_ptr,
                      sp: mp_srcptr,
                      n: size_t,
                      count: c_uint)
                      -> limb_t;
    #[link_name = "__gmpn_cmp"]
    pub fn mpn_cmp(s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> c_int;
    #[link_name = "__gmpn_zero_p"]
    pub fn mpn_zero_p(sp: mp_srcptr, n: size_t) -> c_int;
    #[link_name = "__gmpn_gcd"]
    pub fn mpn_gcd(rp: mp_ptr,
                   xp: mp_ptr,
                   xn: size_t,
                   yp: mp_ptr,
                   yn: size_t)
                   -> size_t;
    #[link_name = "__gmpn_gcd_1"]
    pub fn mpn_gcd_1(xp: mp_srcptr, xn: size_t, yimb: limb_t) -> limb_t;
    #[link_name = "__gmpn_gcdext"]
    pub fn mpn_gcdext(gp: mp_ptr,
                      sp: mp_ptr,
                      sn: *mut size_t,
                      up: mp_ptr,
                      un: size_t,
                      vp: mp_ptr,
                      vn: size_t)
                      -> size_t;
    #[link_name = "__gmpn_sqrtrem"]
    pub fn mpn_sqrtrem(r1p: mp_ptr,
                       r2p: mp_ptr,
                       sp: mp_srcptr,
                       n: size_t)
                       -> size_t;
    #[link_name = "__gmpn_sizeinbase"]
    pub fn mpn_sizeinbase(xp: mp_srcptr, n: size_t, base: c_int) -> usize;
    #[link_name = "__gmpn_get_str"]
    pub fn mpn_get_str(str: *mut c_uchar,
                       base: c_int,
                       s1p: mp_ptr,
                       s1n: size_t)
                       -> usize;
    #[link_name = "__gmpn_set_str"]
    pub fn mpn_set_str(rp: mp_ptr,
                       str: *const c_uchar,
                       strsize: usize,
                       base: c_int)
                       -> size_t;
    #[link_name = "__gmpn_scan0"]
    pub fn mpn_scan0(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    #[link_name = "__gmpn_scan1"]
    pub fn mpn_scan1(s1p: mp_srcptr, bit: bitcnt_t) -> bitcnt_t;
    #[link_name = "__gmpn_random"]
    pub fn mpn_random(r1p: mp_ptr, r1n: size_t);
    #[link_name = "__gmpn_random2"]
    pub fn mpn_random2(r1p: mp_ptr, r1n: size_t);
    #[link_name = "__gmpn_popcount"]
    pub fn mpn_popcount(s1p: mp_srcptr, n: size_t) -> bitcnt_t;
    #[link_name = "__gmpn_hamdist"]
    pub fn mpn_hamdist(s1p: mp_srcptr, s2p: mp_srcptr, n: size_t) -> bitcnt_t;
    #[link_name = "__gmpn_perfect_square_p"]
    pub fn mpn_perfect_square_p(s1p: mp_srcptr, n: size_t) -> c_int;
    #[link_name = "__gmpn_and_n"]
    pub fn mpn_and_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_ior_n"]
    pub fn mpn_ior_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_xor_n"]
    pub fn mpn_xor_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_andn_n"]
    pub fn mpn_andn_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_iorn_n"]
    pub fn mpn_iorn_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_nand_n"]
    pub fn mpn_nand_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_nior_n"]
    pub fn mpn_nior_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_xnor_n"]
    pub fn mpn_xnor_n(rp: mp_ptr, s1p: mp_srcptr, s2p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_com"]
    pub fn mpn_com(rp: mp_ptr, sp: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_copyi"]
    pub fn mpn_copyi(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_copyd"]
    pub fn mpn_copyd(rp: mp_ptr, s1p: mp_srcptr, n: size_t);
    #[link_name = "__gmpn_zero"]
    pub fn mpn_zero(rp: mp_ptr, n: size_t);

    // Low-level functions for cryptography

    #[link_name = "__gmpn_cnd_add_n"]
    pub fn mpn_cnd_add_n(cnd: limb_t,
                         rp: mp_ptr,
                         s1p: mp_srcptr,
                         s2p: mp_srcptr,
                         n: size_t)
                         -> limb_t;
    #[link_name = "__gmpn_cnd_sub_n"]
    pub fn mpn_cnd_sub_n(cnd: limb_t,
                         rp: mp_ptr,
                         s1p: mp_srcptr,
                         s2p: mp_srcptr,
                         n: size_t)
                         -> limb_t;
    #[link_name = "__gmpn_sec_add_1"]
    pub fn mpn_sec_add_1(rp: mp_ptr,
                         ap: mp_srcptr,
                         n: size_t,
                         b: limb_t,
                         tp: mp_ptr)
                         -> limb_t;
    #[link_name = "__gmpn_sec_add_1_itch"]
    pub fn mpn_sec_add_1_itch(n: size_t) -> size_t;
    #[link_name = "__gmpn_sec_sub_1"]
    pub fn mpn_sec_sub_1(rp: mp_ptr,
                         ap: mp_srcptr,
                         n: size_t,
                         b: limb_t,
                         tp: mp_ptr)
                         -> limb_t;
    #[link_name = "__gmpn_sec_sub_1_itch"]
    pub fn mpn_sec_sub_1_itch(n: size_t) -> size_t;
    #[link_name = "__gmpn_cnd_swap"]
    pub fn mpn_cnd_swap(cnd: limb_t,
                        ap: *mut limb_t,
                        bp: *mut limb_t,
                        n: size_t);
    #[link_name = "__gmpn_sec_mul"]
    pub fn mpn_sec_mul(rp: mp_ptr,
                       ap: mp_srcptr,
                       an: size_t,
                       bp: mp_srcptr,
                       bn: size_t,
                       tp: mp_ptr);
    #[link_name = "__gmpn_sec_mul_itch"]
    pub fn mpn_sec_mul_itch(an: size_t, bn: size_t) -> size_t;
    #[link_name = "__gmpn_sec_sqr"]
    pub fn mpn_sec_sqr(rp: mp_ptr, ap: mp_srcptr, an: size_t, tp: mp_ptr);
    #[link_name = "__gmpn_sec_sqr_itch"]
    pub fn mpn_sec_sqr_itch(an: size_t) -> size_t;
    #[link_name = "__gmpn_sec_powm"]
    pub fn mpn_sec_powm(rp: mp_ptr,
                        bp: mp_srcptr,
                        bn: size_t,
                        ep: mp_srcptr,
                        enb: bitcnt_t,
                        mp: mp_srcptr,
                        n: size_t,
                        tp: mp_ptr);
    #[link_name = "__gmpn_sec_powm_itch"]
    pub fn mpn_sec_powm_itch(bn: size_t, enb: bitcnt_t, n: size_t) -> size_t;
    #[link_name = "__gmpn_sec_tabselect"]
    pub fn mpn_sec_tabselect(rp: *mut limb_t,
                             tab: *const limb_t,
                             n: size_t,
                             nents: size_t,
                             which: size_t);
    #[link_name = "__gmpn_sec_div_qr"]
    pub fn mpn_sec_div_qr(qp: mp_ptr,
                          np: mp_ptr,
                          nn: size_t,
                          dp: mp_srcptr,
                          dn: size_t,
                          tp: mp_ptr)
                          -> limb_t;
    #[link_name = "__gmpn_sec_div_qr_itch"]
    pub fn mpn_sec_div_qr_itch(nn: size_t, dn: size_t) -> size_t;
    #[link_name = "__gmpn_sec_div_r"]
    pub fn mpn_sec_div_r(np: mp_ptr,
                         nn: size_t,
                         dp: mp_srcptr,
                         dn: size_t,
                         tp: mp_ptr);
    #[link_name = "__gmpn_sec_div_r_itch"]
    pub fn mpn_sec_div_r_itch(nn: size_t, dn: size_t) -> size_t;
    #[link_name = "__gmpn_sec_invert"]
    pub fn mpn_sec_invert(rp: mp_ptr,
                          ap: mp_ptr,
                          mp: mp_srcptr,
                          n: size_t,
                          nbcnt: bitcnt_t,
                          tp: mp_ptr)
                          -> c_int;
    #[link_name = "__gmpn_sec_invert_itch"]
    pub fn mpn_sec_invert_itch(n: size_t) -> size_t;
}

// Random Numbers

extern "C" {
    // Random State Initialization

    #[link_name = "__gmp_randinit_default"]
    pub fn randinit_default(state: randstate_ptr);
    #[link_name = "__gmp_randinit_mt"]
    pub fn randinit_mt(state: randstate_ptr);
    #[link_name = "__gmp_randinit_lc_2exp"]
    pub fn randinit_lc_2exp(state: randstate_ptr,
                            a: mpz_srcptr,
                            c: c_ulong,
                            m2exp: bitcnt_t);
    #[link_name = "__gmp_randinit_lc_2exp_size"]
    pub fn randinit_lc_2exp_size(state: randstate_ptr,
                                 size: bitcnt_t)
                                 -> c_int;
    #[link_name = "__gmp_randinit_set"]
    pub fn randinit_set(rop: randstate_ptr, op: randstate_srcptr);
    #[link_name = "__gmp_randclear"]
    pub fn randclear(state: randstate_ptr);

    // Random State Seeding

    #[link_name = "__gmp_randseed"]
    pub fn randseed(state: randstate_ptr, seed: mpz_srcptr);
    #[link_name = "__gmp_randseed_ui"]
    pub fn randseed_ui(state: randstate_ptr, seed: c_ulong);

    // Random State Miscellaneous

    #[link_name = "__gmp_urandomb_ui"]
    pub fn urandomb_ui(state: randstate_ptr, n: c_ulong) -> c_ulong;
    #[link_name = "__gmp_urandomm_ui"]
    pub fn urandomm_ui(state: randstate_ptr, n: c_ulong) -> c_ulong;
}

// Formatted Output

extern "C" {
    #[link_name = "__gmp_printf"]
    pub fn printf(fmt: *const c_char, ...) -> c_int;
    #[link_name = "__gmp_sprintf"]
    pub fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    #[link_name = "__gmp_snprintf"]
    pub fn snprintf(buf: *mut c_char,
                    size: usize,
                    fmt: *const c_char,
                    ...)
                    -> c_int;
    #[link_name = "__gmp_asprintf"]
    pub fn asprintf(pp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
}

// Formatted Input

extern "C" {
    #[link_name = "__gmp_scanf"]
    pub fn scanf(fmt: *const c_char, ...) -> c_int;
    #[link_name = "__gmp_sscanf"]
    pub fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
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
extern "C" {
    #[link_name = "__gmp_set_memory_functions"]
    pub fn set_memory_functions(alloc_func_ptr: allocate_function,
                                realloc_func_ptr: reallocate_function,
                                free_func_ptr: free_function);
    #[link_name = "__gmp_get_memory_functions"]
    pub fn get_memory_functions(alloc_func_ptr: *mut allocate_function,
                                realloc_func_ptr: *mut reallocate_function,
                                free_func_ptr: *mut free_function);
}
