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

use gmp;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum rnd_t {
    RNDN = 0,
    RNDZ = 1,
    RNDU = 2,
    RNDD = 3,
    RNDA = 4,
    RNDF = 5,
    RNDNA = -1,
}

pub type prec_t = c_long;
pub type uprec_t = c_ulong;
pub type sign_t = c_int;
pub type exp_t = c_long;
pub type uexp_t = c_ulong;

pub const PREC_MIN: prec_t = 2;
pub const PREC_MAX: prec_t = (!(0 as uprec_t) >> 1) as prec_t;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpfr_t {
    pub prec: prec_t,
    pub sign: sign_t,
    pub exp: exp_t,
    pub d: *mut gmp::limb_t,
}

pub const NAN_KIND: c_int = 0;
pub const INF_KIND: c_int = 1;
pub const ZERO_KIND: c_int = 2;
pub const REGULAR_KIND: c_int = 3;

// Types for function declarations in this file.

type mpz_srcptr = *const gmp::mpz_t;
type mpz_ptr = *mut gmp::mpz_t;
type mpq_srcptr = *const gmp::mpq_t;
type mpf_srcptr = *const gmp::mpf_t;
type mpf_ptr = *mut gmp::mpf_t;
type randstate_ptr = *mut gmp::randstate_t;
type mpfr_ptr = *mut mpfr_t;
type mpfr_srcptr = *const mpfr_t;

extern "C" {
    // Initialization Functions

    #[link_name = "mpfr_init2"]
    pub fn init2(x: mpfr_ptr, prec: prec_t);
    #[link_name = "mpfr_inits2"]
    pub fn inits2(prec: prec_t, x: mpfr_ptr, ...);
    #[link_name = "mpfr_clear"]
    pub fn clear(x: mpfr_ptr);
    #[link_name = "mpfr_clears"]
    pub fn clears(x: mpfr_ptr, ...);
    #[link_name = "mpfr_init"]
    pub fn init(x: mpfr_ptr);
    #[link_name = "mpfr_inits"]
    pub fn inits(x: mpfr_ptr, ...);
    #[link_name = "mpfr_set_default_prec"]
    pub fn set_default_prec(prec: prec_t);
    #[link_name = "mpfr_get_default_prec"]
    pub fn get_default_prec() -> prec_t;
    #[link_name = "mpfr_set_prec"]
    pub fn set_prec(x: mpfr_ptr, prec: prec_t);
    #[link_name = "mpfr_get_prec"]
    pub fn get_prec(x: mpfr_srcptr) -> prec_t;

    // Assignment Functions

    #[link_name = "mpfr_set"]
    pub fn set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_ui"]
    pub fn set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_si"]
    pub fn set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_flt"]
    pub fn set_flt(rop: mpfr_ptr, op: f32, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_d"]
    pub fn set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_ld"]
    pub fn set_ld(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_z"]
    pub fn set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_q"]
    pub fn set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_f"]
    pub fn set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_set_ui_2exp"]
    pub fn set_ui_2exp(rop: mpfr_ptr,
                       op: c_ulong,
                       e: exp_t,
                       rnd: rnd_t)
                       -> c_int;
    #[link_name = "mpfr_set_si_2exp"]
    pub fn set_si_2exp(rop: mpfr_ptr,
                       op: c_long,
                       e: exp_t,
                       rnd: rnd_t)
                       -> c_int;
    #[link_name = "mpfr_set_z_2exp"]
    pub fn set_z_2exp(rop: mpfr_ptr,
                      op: mpz_srcptr,
                      e: exp_t,
                      rnd: rnd_t)
                      -> c_int;
    #[link_name = "mpfr_set_str"]
    pub fn set_str(rop: mpfr_ptr,
                   s: *const c_char,
                   base: c_int,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_strtofr"]
    pub fn strtofr(rop: mpfr_ptr,
                   nptr: *const c_char,
                   endptr: *mut *mut c_char,
                   base: c_int,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_set_nan"]
    pub fn set_nan(x: mpfr_ptr);
    #[link_name = "mpfr_set_inf"]
    pub fn set_inf(x: mpfr_ptr, sign: c_int);
    #[link_name = "mpfr_set_zero"]
    pub fn set_zero(x: mpfr_ptr, sign: c_int);
    #[link_name = "mpfr_swap"]
    pub fn swap(x: mpfr_ptr, y: mpfr_ptr);
}

// Combined Initialization and Assignment Functions

#[inline]
pub unsafe fn init_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int {
    init(rop);
    set_ui(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int {
    init(rop);
    set_si(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int {
    init(rop);
    set_d(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_ld(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int {
    init(rop);
    set_ld(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_z(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_q(rop, op, rnd)
}
#[inline]
pub unsafe fn init_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int {
    init(rop);
    set_f(rop, op, rnd)
}
extern "C" {
    #[link_name = "mpfr_init_set_str"]
    pub fn init_set_str(x: mpfr_ptr,
                        s: *const c_char,
                        base: c_int,
                        rnd: rnd_t)
                        -> c_int;

    // Conversion Functions

    #[link_name = "mpfr_get_flt"]
    pub fn get_flt(op: mpfr_srcptr, rnd: rnd_t) -> f32;
    #[link_name = "mpfr_get_d"]
    pub fn get_d(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    #[link_name = "mpfr_get_ld"]
    pub fn get_ld(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    #[link_name = "mpfr_get_si"]
    pub fn get_si(op: mpfr_srcptr, rnd: rnd_t) -> c_long;
    #[link_name = "mpfr_get_ui"]
    pub fn get_ui(op: mpfr_srcptr, rnd: rnd_t) -> c_ulong;
    #[link_name = "mpfr_get_d_2exp"]
    pub fn get_d_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: rnd_t) -> f64;
    #[link_name = "mpfr_get_ld_2exp"]
    pub fn get_ld_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: rnd_t) -> f64;
    #[link_name = "mpfr_frexp"]
    pub fn frexp(exp: *mut exp_t,
                 y: mpfr_ptr,
                 x: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_get_z_2exp"]
    pub fn get_z_2exp(rop: mpz_ptr, op: mpfr_srcptr) -> exp_t;
    #[link_name = "mpfr_get_z"]
    pub fn get_z(z: mpz_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_get_f"]
    pub fn get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_get_str"]
    pub fn get_str(str: *mut c_char,
                   expptr: *mut exp_t,
                   b: c_int,
                   n: usize,
                   op: mpfr_srcptr,
                   rnd: rnd_t)
                   -> *mut c_char;
    #[link_name = "mpfr_free_str"]
    pub fn free_str(str: *mut c_char);
    #[link_name = "mpfr_fits_ulong_p"]
    pub fn fits_ulong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_slong_p"]
    pub fn fits_slong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_uint_p"]
    pub fn fits_uint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_sint_p"]
    pub fn fits_sint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_ushort_p"]
    pub fn fits_ushort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_sshort_p"]
    pub fn fits_sshort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_uintmax_p"]
    pub fn fits_uintmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fits_intmax_p"]
    pub fn fits_intmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;

    // Basic Arithmetic Functions

    #[link_name = "mpfr_add"]
    pub fn add(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_add_ui"]
    pub fn add_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_add_si"]
    pub fn add_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_add_d"]
    pub fn add_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_add_z"]
    pub fn add_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_add_q"]
    pub fn add_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sub"]
    pub fn sub(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_ui_sub"]
    pub fn ui_sub(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_sub_ui"]
    pub fn sub_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_si_sub"]
    pub fn si_sub(rop: mpfr_ptr,
                  op1: c_long,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_sub_si"]
    pub fn sub_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_d_sub"]
    pub fn d_sub(rop: mpfr_ptr,
                 op1: f64,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sub_d"]
    pub fn sub_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_z_sub"]
    pub fn z_sub(rop: mpfr_ptr,
                 op1: mpz_srcptr,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sub_z"]
    pub fn sub_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sub_q"]
    pub fn sub_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_mul"]
    pub fn mul(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_mul_ui"]
    pub fn mul_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_mul_si"]
    pub fn mul_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_mul_d"]
    pub fn mul_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_mul_z"]
    pub fn mul_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_mul_q"]
    pub fn mul_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sqr"]
    pub fn sqr(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_div"]
    pub fn div(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_ui_div"]
    pub fn ui_div(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_div_ui"]
    pub fn div_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_si_div"]
    pub fn si_div(rop: mpfr_ptr,
                  op1: c_long,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_div_si"]
    pub fn div_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_d_div"]
    pub fn d_div(rop: mpfr_ptr,
                 op1: f64,
                 op2: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_div_d"]
    pub fn div_d(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: f64,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_div_z"]
    pub fn div_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_div_q"]
    pub fn div_q(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpq_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_sqrt"]
    pub fn sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_sqrt_ui"]
    pub fn sqrt_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_rec_sqrt"]
    pub fn rec_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_cbrt"]
    pub fn cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_root"]
    pub fn root(rop: mpfr_ptr,
                op: mpfr_srcptr,
                k: c_ulong,
                rnd: rnd_t)
                -> c_int;
    #[link_name = "mpfr_pow"]
    pub fn pow(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_pow_ui"]
    pub fn pow_ui(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_pow_si"]
    pub fn pow_si(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_pow_z"]
    pub fn pow_z(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_ui_pow_ui"]
    pub fn ui_pow_ui(rop: mpfr_ptr,
                     op1: c_ulong,
                     op2: c_ulong,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpfr_ui_pow"]
    pub fn ui_pow(rop: mpfr_ptr,
                  op1: c_ulong,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_neg"]
    pub fn neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_abs"]
    pub fn abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_dim"]
    pub fn dim(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_mul_2ui"]
    pub fn mul_2ui(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_mul_2si"]
    pub fn mul_2si(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_div_2ui"]
    pub fn div_2ui(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_div_2si"]
    pub fn div_2si(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;

    // Comparison Functions

    #[link_name = "mpfr_cmp"]
    pub fn cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_cmp_ui"]
    pub fn cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;
    #[link_name = "mpfr_cmp_si"]
    pub fn cmp_si(op1: mpfr_srcptr, op2: c_long) -> c_int;
    #[link_name = "mpfr_cmp_d"]
    pub fn cmp_d(op1: mpfr_srcptr, op2: f64) -> c_int;
    #[link_name = "mpfr_cmp_ld"]
    pub fn cmp_ld(op1: mpfr_srcptr, op2: f64) -> c_int;
    #[link_name = "mpfr_cmp_z"]
    pub fn cmp_z(op1: mpfr_srcptr, op2: mpz_srcptr) -> c_int;
    #[link_name = "mpfr_cmp_q"]
    pub fn cmp_q(op1: mpfr_srcptr, op2: mpq_srcptr) -> c_int;
    #[link_name = "mpfr_cmp_f"]
    pub fn cmp_f(op1: mpfr_srcptr, op2: mpf_srcptr) -> c_int;
    #[link_name = "mpfr_cmp_ui_2exp"]
    pub fn cmp_ui_2exp(op1: mpfr_srcptr, op2: c_ulong, e: exp_t) -> c_int;
    #[link_name = "mpfr_cmp_si_2exp"]
    pub fn cmp_si_2exp(op1: mpfr_srcptr, op2: c_long, e: exp_t) -> c_int;
    #[link_name = "mpfr_cmpabs"]
    pub fn cmpabs(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_nan_p"]
    pub fn nan_p(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_inf_p"]
    pub fn inf_p(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_number_p"]
    pub fn number_p(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_zero_p"]
    pub fn zero_p(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_regular_p"]
    pub fn regular_p(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_sgn"]
    pub fn sgn(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_greater_p"]
    pub fn greater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_greaterequal_p"]
    pub fn greaterequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_less_p"]
    pub fn less_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_lessequal_p"]
    pub fn lessequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_equal_p"]
    pub fn equal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_lessgreater_p"]
    pub fn lessgreater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_unordered_p"]
    pub fn unordered_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;

    // Special Functions

    #[link_name = "mpfr_log"]
    pub fn log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_log2"]
    pub fn log2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_log10"]
    pub fn log10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_exp"]
    pub fn exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_exp2"]
    pub fn exp2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_exp10"]
    pub fn exp10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_cos"]
    pub fn cos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_sin"]
    pub fn sin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_tan"]
    pub fn tan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_sin_cos"]
    pub fn sin_cos(sop: mpfr_ptr,
                   cop: mpfr_ptr,
                   op: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_sec"]
    pub fn sec(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_csc"]
    pub fn csc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_cot"]
    pub fn cot(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_acos"]
    pub fn acos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_asin"]
    pub fn asin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_atan"]
    pub fn atan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_atan2"]
    pub fn atan2(rop: mpfr_ptr,
                 y: mpfr_srcptr,
                 x: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_cosh"]
    pub fn cosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_sinh"]
    pub fn sinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_tanh"]
    pub fn tanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_sinh_cosh"]
    pub fn sinh_cosh(sop: mpfr_ptr,
                     cop: mpfr_ptr,
                     op: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpfr_sech"]
    pub fn sech(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_csch"]
    pub fn csch(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_coth"]
    pub fn coth(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_acosh"]
    pub fn acosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_asinh"]
    pub fn asinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_atanh"]
    pub fn atanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fac_ui"]
    pub fn fac_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_log1p"]
    pub fn log1p(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_expm1"]
    pub fn expm1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_eint"]
    pub fn eint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_li2"]
    pub fn li2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_gamma"]
    pub fn gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_lngamma"]
    pub fn lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_lgamma"]
    pub fn lgamma(rop: mpfr_ptr,
                  signp: *mut c_int,
                  op: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_digamma"]
    pub fn digamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_zeta"]
    pub fn zeta(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_zeta_ui"]
    pub fn zeta_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_erf"]
    pub fn erf(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_erfc"]
    pub fn erfc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_j0"]
    pub fn j0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_j1"]
    pub fn j1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_jn"]
    pub fn jn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_y0"]
    pub fn y0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_y1"]
    pub fn y1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_yn"]
    pub fn yn(rop: mpfr_ptr, n: c_long, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_fma"]
    pub fn fma(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               op3: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_fms"]
    pub fn fms(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               op3: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_agm"]
    pub fn agm(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_hypot"]
    pub fn hypot(rop: mpfr_ptr,
                 x: mpfr_srcptr,
                 y: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpfr_ai"]
    pub fn ai(rop: mpfr_ptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_const_log2"]
    pub fn const_log2(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_const_pi"]
    pub fn const_pi(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_const_euler"]
    pub fn const_euler(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_const_catalan"]
    pub fn const_catalan(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_free_cache"]
    pub fn free_cache();
    #[link_name = "mpfr_sum"]
    pub fn sum(rop: mpfr_ptr,
               tab: *mut mpfr_ptr,
               n: c_ulong,
               rnd: rnd_t)
               -> c_int;

    // Formatted Output Functions

    #[link_name = "mpfr_printf"]
    pub fn printf(template: *const c_char, ...) -> c_int;
    #[link_name = "mpfr_sprintf"]
    pub fn sprintf(buf: *mut c_char, template: *const c_char, ...) -> c_int;
    #[link_name = "mpfr_snprintf"]
    pub fn snprintf(buf: *mut c_char,
                    n: usize,
                    template: *const c_char,
                    ...)
                    -> c_int;
    #[link_name = "mpfr_asprintf"]
    pub fn asprintf(str: *mut *mut c_char,
                    template: *const c_char,
                    ...)
                    -> c_int;

    // Integer and Remainder Related Functions

    #[link_name = "mpfr_rint"]
    pub fn rint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_ceil"]
    pub fn ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_floor"]
    pub fn floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_round"]
    pub fn round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_trunc"]
    pub fn trunc(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_rint_ceil"]
    pub fn rint_ceil(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_rint_floor"]
    pub fn rint_floor(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_rint_round"]
    pub fn rint_round(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_rint_trunc"]
    pub fn rint_trunc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_frac"]
    pub fn frac(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_modf"]
    pub fn modf(iop: mpfr_ptr,
                fop: mpfr_ptr,
                op: mpfr_srcptr,
                rnd: rnd_t)
                -> c_int;
    #[link_name = "mpfr_fmod"]
    pub fn fmod(r: mpfr_ptr,
                x: mpfr_srcptr,
                y: mpfr_srcptr,
                rnd: rnd_t)
                -> c_int;
    #[link_name = "mpfr_remainder"]
    pub fn remainder(r: mpfr_ptr,
                     x: mpfr_srcptr,
                     y: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpfr_remquo"]
    pub fn remquo(r: mpfr_ptr,
                  q: *mut c_long,
                  x: mpfr_srcptr,
                  y: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpfr_integer_p"]
    pub fn integer_p(op: mpfr_srcptr) -> c_int;

    // Rounding Related Functions

    #[link_name = "mpfr_set_default_rounding_mode"]
    pub fn set_default_rounding_mode(rnd: rnd_t);
    #[link_name = "mpfr_get_default_rounding_mode"]
    pub fn get_default_rounding_mode() -> rnd_t;
    #[link_name = "mpfr_prec_round"]
    pub fn prec_round(x: mpfr_ptr, prec: prec_t, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_can_round"]
    pub fn can_round(b: mpfr_srcptr,
                     err: exp_t,
                     rnd1: rnd_t,
                     rnd2: rnd_t,
                     prec: prec_t)
                     -> c_int;
    #[link_name = "mpfr_min_prec"]
    pub fn min_prec(x: mpfr_srcptr) -> prec_t;
    #[link_name = "mpfr_print_rnd_mode"]
    pub fn print_rnd_mode(rnd: rnd_t) -> *const c_char;

    // Miscellaneous Functions

    #[link_name = "mpfr_nexttoward"]
    pub fn nexttoward(x: mpfr_ptr, y: mpfr_srcptr);
    #[link_name = "mpfr_nextabove"]
    pub fn nextabove(x: mpfr_ptr);
    #[link_name = "mpfr_nextbelow"]
    pub fn nextbelow(x: mpfr_ptr);
    #[link_name = "mpfr_min"]
    pub fn min(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_max"]
    pub fn max(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpfr_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpfr_urandomb"]
    pub fn urandomb(rop: mpfr_ptr, state: randstate_ptr) -> c_int;
    #[link_name = "mpfr_urandom"]
    pub fn urandom(rop: mpfr_ptr, state: randstate_ptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_grandom"]
    pub fn grandom(rop1: mpfr_ptr,
                   rop2: mpfr_ptr,
                   state: randstate_ptr,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_get_exp"]
    pub fn get_exp(x: mpfr_srcptr) -> exp_t;
    #[link_name = "mpfr_set_exp"]
    pub fn set_exp(x: mpfr_ptr, e: exp_t) -> c_int;
    #[link_name = "mpfr_signbit"]
    pub fn signbit(op: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_setsign"]
    pub fn setsign(rop: mpfr_ptr,
                   op: mpfr_srcptr,
                   s: c_int,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpfr_copysign"]
    pub fn copysign(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: mpfr_srcptr,
                    rnd: rnd_t)
                    -> c_int;
    #[link_name = "mpfr_get_version"]
    pub fn get_version() -> *const c_char;
}
pub const VERSION: c_int = (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) |
                           VERSION_PATCHLEVEL;
pub const VERSION_MAJOR: c_int = 3;
pub const VERSION_MINOR: c_int = 1;
pub const VERSION_PATCHLEVEL: c_int = 5;
pub const VERSION_STRING: *const c_char = b"3.1.5\0" as *const u8 as
                                          *const c_char;
#[inline]
pub fn VERSION_NUM(major: c_int, minor: c_int, patchlevel: c_int) -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
extern "C" {
    #[link_name = "mpfr_get_patches"]
    pub fn get_patches() -> *const c_char;
    #[link_name = "mpfr_buildopt_tls_p"]
    pub fn buildopt_tls_p() -> c_int;
    #[link_name = "mpfr_buildopt_decimal_p"]
    pub fn buildopt_decimal_p() -> c_int;
    #[link_name = "mpfr_buildopt_gmpinternals_p"]
    pub fn buildopt_gmpinternals_p() -> c_int;
    #[link_name = "mpfr_buildopt_tune_case"]
    pub fn buildopt_tune_case() -> *const c_char;

    // Exception Related Functions

    #[link_name = "mpfr_get_emin"]
    pub fn get_emin() -> exp_t;
    #[link_name = "mpfr_get_emax"]
    pub fn get_emax() -> exp_t;
    #[link_name = "mpfr_set_emin"]
    pub fn set_emin(exp: exp_t) -> c_int;
    #[link_name = "mpfr_set_emax"]
    pub fn set_emax(exp: exp_t) -> c_int;
    #[link_name = "mpfr_get_emin_min"]
    pub fn get_emin_min() -> exp_t;
    #[link_name = "mpfr_get_emin_max"]
    pub fn get_emin_max() -> exp_t;
    #[link_name = "mpfr_get_emax_min"]
    pub fn get_emax_min() -> exp_t;
    #[link_name = "mpfr_get_emax_max"]
    pub fn get_emax_max() -> exp_t;
    #[link_name = "mpfr_check_range"]
    pub fn check_range(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_subnormalize"]
    pub fn subnormalize(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    #[link_name = "mpfr_clear_underflow"]
    pub fn clear_underflow();
    #[link_name = "mpfr_clear_overflow"]
    pub fn clear_overflow();
    #[link_name = "mpfr_clear_divby0"]
    pub fn clear_divby0();
    #[link_name = "mpfr_clear_nanflag"]
    pub fn clear_nanflag();
    #[link_name = "mpfr_clear_inexflag"]
    pub fn clear_inexflag();
    #[link_name = "mpfr_clear_erangeflag"]
    pub fn clear_erangeflag();
    #[link_name = "mpfr_set_underflow"]
    pub fn set_underflow();
    #[link_name = "mpfr_set_overflow"]
    pub fn set_overflow();
    #[link_name = "mpfr_set_divby0"]
    pub fn set_divby0();
    #[link_name = "mpfr_set_nanflag"]
    pub fn set_nanflag();
    #[link_name = "mpfr_set_inexflag"]
    pub fn set_inexflag();
    #[link_name = "mpfr_set_erangeflag"]
    pub fn set_erangeflag();
    #[link_name = "mpfr_clear_flags"]
    pub fn clear_flags();
    #[link_name = "mpfr_underflow_p"]
    pub fn underflow_p() -> c_int;
    #[link_name = "mpfr_overflow_p"]
    pub fn overflow_p() -> c_int;
    #[link_name = "mpfr_divby0_p"]
    pub fn divby0_p() -> c_int;
    #[link_name = "mpfr_nanflag_p"]
    pub fn nanflag_p() -> c_int;
    #[link_name = "mpfr_inexflag_p"]
    pub fn inexflag_p() -> c_int;
    #[link_name = "mpfr_erangeflag_p"]
    pub fn erangeflag_p() -> c_int;

    // Compatibility with MPF

    #[link_name = "mpfr_set_prec_raw"]
    pub fn set_prec_raw(x: mpfr_ptr, prec: prec_t);
    #[link_name = "mpfr_eq"]
    pub fn eq(op1: mpfr_srcptr, op2: mpfr_srcptr, op3: c_ulong) -> c_int;
    #[link_name = "mpfr_reldiff"]
    pub fn reldiff(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t);
    #[link_name = "mpfr_mul_2exp"]
    pub fn mul_2exp(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: c_ulong,
                    rnd: rnd_t)
                    -> c_int;
    #[link_name = "mpfr_div_2exp"]
    pub fn div_2exp(rop: mpfr_ptr,
                    op1: mpfr_srcptr,
                    op2: c_ulong,
                    rnd: rnd_t)
                    -> c_int;

    // Custom Interface

    #[link_name = "mpfr_custom_get_size"]
    pub fn custom_get_size(prec: prec_t) -> usize;
    #[link_name = "mpfr_custom_init"]
    pub fn custom_init(significand: *mut c_void, prec: prec_t);
    #[link_name = "mpfr_custom_init_set"]
    pub fn custom_init_set(x: mpfr_ptr,
                           kind: c_int,
                           exp: exp_t,
                           prec: prec_t,
                           significand: *mut c_void);
    #[link_name = "mpfr_custom_get_kind"]
    pub fn custom_get_kind(x: mpfr_srcptr) -> c_int;
    #[link_name = "mpfr_custom_get_significand"]
    pub fn custom_get_significand(x: mpfr_srcptr) -> *mut c_void;
    #[link_name = "mpfr_custom_get_exp"]
    pub fn custom_get_exp(x: mpfr_srcptr) -> exp_t;
    #[link_name = "mpfr_custom_move"]
    pub fn custom_move(x: mpfr_ptr, new_position: *mut c_void);
}

#[cfg(test)]
mod tests {
    use mpfr;
    use std::ffi::CStr;
    use std::mem;

    #[test]
    fn check_version() {
        let version = "3.1.5";
        let from_fn = unsafe { CStr::from_ptr(mpfr::get_version()) };
        let from_constants = format!("{}.{}.{}",
                                     mpfr::VERSION_MAJOR,
                                     mpfr::VERSION_MINOR,
                                     mpfr::VERSION_PATCHLEVEL);
        let from_const_string = unsafe { CStr::from_ptr(mpfr::VERSION_STRING) };
        assert!(from_fn.to_str().unwrap() == version);
        assert!(from_constants == version);
        assert!(from_const_string.to_str().unwrap() == version);
    }

    #[test]
    fn it_runs() {
        let d: f64 = 1.0 / 3.0;
        unsafe {
            let mut fr: mpfr::mpfr_t = mem::uninitialized();
            let ptr = &mut fr as *mut _;
            mpfr::init2(ptr, 53);
            assert!(mpfr::set_d(ptr, d, mpfr::rnd_t::RNDN) == 0);
            assert!(mpfr::get_d(ptr, mpfr::rnd_t::RNDN) == d);
            mpfr::clear(ptr);
        }
    }
}
