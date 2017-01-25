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

macro_rules! c_fn {
    { $($c:ident
        $( ($($par:ident: $ty:ty),* $(; $dots:tt)*) $(-> $ret:ty)*
        )* ;
    )* } => {
        $(
            $(
                #[link(name = "mpfr", kind = "static")]
                extern "C" {
                    pub fn $c($($par: $ty),* $(, $dots)*) $(-> $ret)*;
                }
            )*
        )*
    };
}

use ::gmp::*;
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

type mpz_srcptr = *const mpz_t;
type mpz_ptr = *mut mpz_t;
type mpq_srcptr = *const mpq_t;
type mpf_srcptr = *const mpf_t;
type mpf_ptr = *mut mpf_t;
type randstate_ptr = *mut gmp_randstate_t;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum mpfr_rnd_t {
    MPFR_RNDN = 0,
    MPFR_RNDZ = 1,
    MPFR_RNDU = 2,
    MPFR_RNDD = 3,
    MPFR_RNDA = 4,
    MPFR_RNDF = 5,
    MPFR_RNDNA = -1,
}
pub const GMP_RNDN: mpfr_rnd_t = mpfr_rnd_t::MPFR_RNDN;
pub const GMP_RNDZ: mpfr_rnd_t = mpfr_rnd_t::MPFR_RNDZ;
pub const GMP_RNDU: mpfr_rnd_t = mpfr_rnd_t::MPFR_RNDU;
pub const GMP_RNDD: mpfr_rnd_t = mpfr_rnd_t::MPFR_RNDD;

pub type mpfr_prec_t = c_long;
pub type mpfr_uprec_t = c_ulong;
pub type mpfr_sign_t = c_int;
pub type mpfr_exp_t = c_long;
pub type mpfr_uexp_t = c_ulong;

pub const MPFR_PREC_MIN: mpfr_prec_t = 2;
pub const MPFR_PREC_MAX: mpfr_prec_t = (!(0 as mpfr_uprec_t) >> 1) as
                                       mpfr_prec_t;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpfr_struct {
    pub _mpfr_prec: mpfr_prec_t,
    pub _mpfr_sign: mpfr_sign_t,
    pub _mpfr_exp: mpfr_exp_t,
    pub _mpfr_d: *mut mp_limb_t,
}
pub type mpfr_t = __mpfr_struct;

type mpfr_ptr = *mut __mpfr_struct;
type mpfr_srcptr = *const __mpfr_struct;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum mpfr_kind_t {
    MPFR_NAN_KIND = 0,
    MPFR_INF_KIND = 1,
    MPFR_ZERO_KIND = 2,
    MPFR_REGULAR_KIND = 3,
}

c_fn! {
    // Initialization Functions
    mpfr_init2(x: mpfr_ptr, prec: mpfr_prec_t);
    mpfr_inits2(prec: mpfr_prec_t, x: mpfr_ptr; ...);
    mpfr_clear(x: mpfr_ptr);
    mpfr_clears(x: mpfr_ptr; ...);
    mpfr_init(x: mpfr_ptr);
    mpfr_inits(x: mpfr_ptr; ...);
    mpfr_set_default_prec(prec: mpfr_prec_t);
    mpfr_get_default_prec() -> mpfr_prec_t;
    mpfr_set_prec(x: mpfr_ptr, prec: mpfr_prec_t);
    mpfr_get_prec(x: mpfr_srcptr) -> mpfr_prec_t;

    // Assignment Functions
    mpfr_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_si(rop: mpfr_ptr, op: c_long, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_flt(rop: mpfr_ptr, op: f32, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_d(rop: mpfr_ptr, op: f64, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_ld(rop: mpfr_ptr, op: f64, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_set_ui_2exp(rop: mpfr_ptr,
                     op: c_ulong,
                     e: mpfr_exp_t,
                     rnd: mpfr_rnd_t)
                     -> c_int;
    mpfr_set_si_2exp(rop: mpfr_ptr,
                     op: c_long,
                     e: mpfr_exp_t,
                     rnd: mpfr_rnd_t)
                     -> c_int;
    mpfr_set_z_2exp(rop: mpfr_ptr,
                    op: mpz_srcptr,
                    e: mpfr_exp_t,
                    rnd: mpfr_rnd_t)
                    -> c_int;
    mpfr_set_str(rop: mpfr_ptr,
                 s: *const c_char,
                 base: c_int,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_strtofr(rop: mpfr_ptr,
                 nptr: *const c_char,
                 endptr: *mut *mut c_char,
                 base: c_int,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_set_nan(x: mpfr_ptr);
    mpfr_set_inf(x: mpfr_ptr, sign: c_int);
    mpfr_set_zero(x: mpfr_ptr, sign: c_int);
    mpfr_swap(x: mpfr_ptr, y: mpfr_ptr);

    // Combined Initialization and Assignment Functions
}
#[inline]
pub unsafe fn mpfr_init_set(rop: mpfr_ptr,
                            op: mpfr_srcptr,
                            rnd: mpfr_rnd_t)
                            -> c_int {
    mpfr_init(rop);
    mpfr_set(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_ui(rop: mpfr_ptr,
                               op: c_ulong,
                               rnd: mpfr_rnd_t)
                               -> c_int {
    mpfr_init(rop);
    mpfr_set_ui(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_si(rop: mpfr_ptr,
                               op: c_long,
                               rnd: mpfr_rnd_t)
                               -> c_int {
    mpfr_init(rop);
    mpfr_set_si(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_d(rop: mpfr_ptr,
                              op: f64,
                              rnd: mpfr_rnd_t)
                              -> c_int {
    mpfr_init(rop);
    mpfr_set_d(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_ld(rop: mpfr_ptr,
                               op: f64,
                               rnd: mpfr_rnd_t)
                               -> c_int {
    mpfr_init(rop);
    mpfr_set_ld(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_z(rop: mpfr_ptr,
                              op: mpz_srcptr,
                              rnd: mpfr_rnd_t)
                              -> c_int {
    mpfr_init(rop);
    mpfr_set_z(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_q(rop: mpfr_ptr,
                              op: mpq_srcptr,
                              rnd: mpfr_rnd_t)
                              -> c_int {
    mpfr_init(rop);
    mpfr_set_q(rop, op, rnd)
}
#[inline]
pub unsafe fn mpfr_init_set_f(rop: mpfr_ptr,
                              op: mpf_srcptr,
                              rnd: mpfr_rnd_t)
                              -> c_int {
    mpfr_init(rop);
    mpfr_set_f(rop, op, rnd)
}
c_fn! {
    mpfr_init_set_str(x: mpfr_ptr,
                      s: *const c_char,
                      base: c_int,
                      rnd: mpfr_rnd_t)
                      -> c_int;

    // Conversion Functions
    mpfr_get_flt(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> f32;
    mpfr_get_d(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> f64;
    mpfr_get_ld(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> f64;
    mpfr_get_si(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_long;
    mpfr_get_ui(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_ulong;
    mpfr_get_d_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> f64;
    mpfr_get_ld_2exp(exp: *mut c_long, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> f64;
    mpfr_frexp(exp: *mut mpfr_exp_t,
               y: mpfr_ptr,
               x: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_get_z_2exp(rop: mpz_ptr, op: mpfr_srcptr) -> mpfr_exp_t;
    mpfr_get_z(z: mpz_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_get_str(str: *mut c_char,
                 expptr: *mut mpfr_exp_t,
                 b: c_int,
                 n: usize,
                 op: mpfr_srcptr,
                 rnd: mpfr_rnd_t)
                 -> *mut c_char;
    mpfr_free_str(str: *mut c_char);
    mpfr_fits_ulong_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_slong_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_uint_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_sint_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_ushort_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_sshort_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_uintmax_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fits_intmax_p(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;

    // Basic Arithmetic Functions
    mpfr_add(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_add_ui(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_ulong,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_add_si(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_long,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_add_d(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: f64,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_add_z(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpz_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_add_q(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpq_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sub(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_ui_sub(rop: mpfr_ptr,
                op1: c_ulong,
                op2: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_sub_ui(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_ulong,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_si_sub(rop: mpfr_ptr,
                op1: c_long,
                op2: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_sub_si(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_long,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_d_sub(rop: mpfr_ptr,
               op1: f64,
               op2: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sub_d(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: f64,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_z_sub(rop: mpfr_ptr,
               op1: mpz_srcptr,
               op2: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sub_z(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpz_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sub_q(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpq_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_mul(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_mul_ui(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_ulong,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_mul_si(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_long,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_mul_d(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: f64,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_mul_z(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpz_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_mul_q(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpq_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sqr(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_div(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_ui_div(rop: mpfr_ptr,
                op1: c_ulong,
                op2: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_div_ui(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_ulong,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_si_div(rop: mpfr_ptr,
                op1: c_long,
                op2: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_div_si(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_long,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_d_div(rop: mpfr_ptr,
               op1: f64,
               op2: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_div_d(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: f64,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_div_z(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpz_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_div_q(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpq_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_sqrt_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    mpfr_rec_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_root(rop: mpfr_ptr,
              op: mpfr_srcptr,
              k: c_ulong,
              rnd: mpfr_rnd_t)
              -> c_int;
    mpfr_pow(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_pow_ui(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_ulong,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_pow_si(rop: mpfr_ptr,
                op1: mpfr_srcptr,
                op2: c_long,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_pow_z(rop: mpfr_ptr,
               op1: mpfr_srcptr,
               op2: mpz_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_ui_pow_ui(rop: mpfr_ptr,
                   op1: c_ulong,
                   op2: c_ulong,
                   rnd: mpfr_rnd_t)
                   -> c_int;
    mpfr_ui_pow(rop: mpfr_ptr,
                op1: c_ulong,
                op2: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_dim(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_mul_2ui(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: c_ulong,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_mul_2si(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: c_long,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_div_2ui(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: c_ulong,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_div_2si(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: c_long,
                 rnd: mpfr_rnd_t)
                 -> c_int;

    // Comparison Functions
    mpfr_cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;
    mpfr_cmp_si(op1: mpfr_srcptr, op2: c_long) -> c_int;
    mpfr_cmp_d(op1: mpfr_srcptr, op2: f64) -> c_int;
    mpfr_cmp_ld(op1: mpfr_srcptr, op2: f64) -> c_int;
    mpfr_cmp_z(op1: mpfr_srcptr, op2: mpz_srcptr) -> c_int;
    mpfr_cmp_q(op1: mpfr_srcptr, op2: mpq_srcptr) -> c_int;
    mpfr_cmp_f(op1: mpfr_srcptr, op2: mpf_srcptr) -> c_int;
    mpfr_cmp_ui_2exp(op1: mpfr_srcptr, op2: c_ulong, e: mpfr_exp_t) -> c_int;
    mpfr_cmp_si_2exp(op1: mpfr_srcptr, op2: c_long, e: mpfr_exp_t) -> c_int;
    mpfr_cmpabs(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_nan_p(op: mpfr_srcptr) -> c_int;
    mpfr_inf_p(op: mpfr_srcptr) -> c_int;
    mpfr_number_p(op: mpfr_srcptr) -> c_int;
    mpfr_zero_p(op: mpfr_srcptr) -> c_int;
    mpfr_regular_p(op: mpfr_srcptr) -> c_int;
    mpfr_sgn(op: mpfr_srcptr) -> c_int;
    mpfr_greater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_greaterequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_less_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_lessequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_equal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_lessgreater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    mpfr_unordered_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;

    // Special Functions
    mpfr_log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_log2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_log10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_exp2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_exp10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_cos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_sin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_tan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_sin_cos(sop: mpfr_ptr,
                 cop: mpfr_ptr,
                 op: mpfr_srcptr,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_sec(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_csc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_cot(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_acos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_asin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_atan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_atan2(rop: mpfr_ptr,
               y: mpfr_srcptr,
               x: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_cosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_sinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_tanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_sinh_cosh(sop: mpfr_ptr,
                   cop: mpfr_ptr,
                   op: mpfr_srcptr,
                   rnd: mpfr_rnd_t)
                   -> c_int;
    mpfr_sech(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_csch(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_coth(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_acosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_asinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_atanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_fac_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    mpfr_log1p(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_expm1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_eint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_li2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_lgamma(rop: mpfr_ptr,
                signp: *mut c_int,
                op: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_digamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_zeta(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_zeta_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    mpfr_erf(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_erfc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_j0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_j1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_jn(rop: mpfr_ptr,
            n: c_long,
            op: mpfr_srcptr,
            rnd: mpfr_rnd_t)
            -> c_int;
    mpfr_y0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_y1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_yn(rop: mpfr_ptr,
            n: c_long,
            op: mpfr_srcptr,
            rnd: mpfr_rnd_t)
            -> c_int;
    mpfr_fma(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             op3: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_fms(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             op3: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_agm(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_hypot(rop: mpfr_ptr,
               x: mpfr_srcptr,
               y: mpfr_srcptr,
               rnd: mpfr_rnd_t)
               -> c_int;
    mpfr_ai(rop: mpfr_ptr, x: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_const_log2(rop: mpfr_ptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_const_pi(rop: mpfr_ptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_const_euler(rop: mpfr_ptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_const_catalan(rop: mpfr_ptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_free_cache();
    mpfr_sum(rop: mpfr_ptr,
             tab: *mut mpfr_ptr,
             n: c_ulong,
             rnd: mpfr_rnd_t)
             -> c_int;

    // Formatted Output Functions
    mpfr_printf(template: *const c_char; ...) -> c_int;
    mpfr_sprintf(buf: *mut c_char, template: *const c_char; ...) -> c_int;
    mpfr_snprintf(buf: *mut c_char,
                  n: usize,
                  template: *const c_char;
                  ...)
                  -> c_int;
    mpfr_asprintf(str: *mut *mut c_char, template: *const c_char; ...) -> c_int;

    // Integer and Remainder Related Functions
    mpfr_rint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    mpfr_floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    mpfr_round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    mpfr_trunc(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    mpfr_rint_ceil(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_rint_floor(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_rint_round(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_rint_trunc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_frac(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_modf(iop: mpfr_ptr,
              fop: mpfr_ptr,
              op: mpfr_srcptr,
              rnd: mpfr_rnd_t)
              -> c_int;
    mpfr_fmod(r: mpfr_ptr,
              x: mpfr_srcptr,
              y: mpfr_srcptr,
              rnd: mpfr_rnd_t)
              -> c_int;
    mpfr_remainder(r: mpfr_ptr,
                   x: mpfr_srcptr,
                   y: mpfr_srcptr,
                   rnd: mpfr_rnd_t)
                   -> c_int;
    mpfr_remquo(r: mpfr_ptr,
                q: *mut c_long,
                x: mpfr_srcptr,
                y: mpfr_srcptr,
                rnd: mpfr_rnd_t)
                -> c_int;
    mpfr_integer_p(op: mpfr_srcptr) -> c_int;

    // Rounding Related Functions
    mpfr_set_default_rounding_mode(rnd: mpfr_rnd_t);
    mpfr_get_default_rounding_mode() -> mpfr_rnd_t;
    mpfr_prec_round(x: mpfr_ptr, prec: mpfr_prec_t, rnd: mpfr_rnd_t) -> c_int;
    mpfr_can_round(b: mpfr_srcptr,
                   err: mpfr_exp_t,
                   rnd1: mpfr_rnd_t,
                   rnd2: mpfr_rnd_t,
                   prec: mpfr_prec_t)
                   -> c_int;
    mpfr_min_prec(x: mpfr_srcptr) -> mpfr_prec_t;
    mpfr_print_rnd_mode(rnd: mpfr_rnd_t) -> *const c_char;

    // Miscellaneous Functions
    mpfr_nexttoward(x: mpfr_ptr, y: mpfr_srcptr);
    mpfr_nextabove(x: mpfr_ptr);
    mpfr_nextbelow(x: mpfr_ptr);
    mpfr_min(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_max(rop: mpfr_ptr,
             op1: mpfr_srcptr,
             op2: mpfr_srcptr,
             rnd: mpfr_rnd_t)
             -> c_int;
    mpfr_urandomb(rop: mpfr_ptr, state: randstate_ptr) -> c_int;
    mpfr_urandom(rop: mpfr_ptr, state: randstate_ptr, rnd: mpfr_rnd_t) -> c_int;
    mpfr_grandom(rop1: mpfr_ptr,
                 rop2: mpfr_ptr,
                 state: randstate_ptr,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_get_exp(x: mpfr_srcptr) -> mpfr_exp_t;
    mpfr_set_exp(x: mpfr_ptr, e: mpfr_exp_t) -> c_int;
    mpfr_signbit(op: mpfr_srcptr) -> c_int;
    mpfr_setsign(rop: mpfr_ptr,
                 op: mpfr_srcptr,
                 s: c_int,
                 rnd: mpfr_rnd_t)
                 -> c_int;
    mpfr_copysign(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: mpfr_srcptr,
                  rnd: mpfr_rnd_t)
                  -> c_int;
    mpfr_get_version() -> *const c_char;
}
pub const MPFR_VERSION: c_int = (MPFR_VERSION_MAJOR << 16) |
                                (MPFR_VERSION_MINOR << 8) |
                                MPFR_VERSION_PATCHLEVEL;
pub const MPFR_VERSION_MAJOR: c_int = 3;
pub const MPFR_VERSION_MINOR: c_int = 1;
pub const MPFR_VERSION_PATCHLEVEL: c_int = 5;
const MPFR_VERSION_BUFFER: &'static [u8] = b"3.1.5\0";
pub const MPFR_VERSION_STRING: *const c_char =
    &MPFR_VERSION_BUFFER[0] as *const _ as *const c_char;
#[inline]
pub fn MPFR_VERSION_NUM(major: c_int,
                        minor: c_int,
                        patchlevel: c_int)
                        -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
c_fn! {
    mpfr_get_patches() -> *const c_char;
    mpfr_buildopt_tls_p() -> c_int;
    mpfr_buildopt_decimal_p() -> c_int;
    mpfr_buildopt_gmpinternals_p() -> c_int;
    mpfr_buildopt_tune_case() -> *const c_char;

    // Exception Related Functions
    mpfr_get_emin() -> mpfr_exp_t;
    mpfr_get_emax() -> mpfr_exp_t;
    mpfr_set_emin(exp: mpfr_exp_t) -> c_int;
    mpfr_set_emax(exp: mpfr_exp_t) -> c_int;
    mpfr_get_emin_min() -> mpfr_exp_t;
    mpfr_get_emin_max() -> mpfr_exp_t;
    mpfr_get_emax_min() -> mpfr_exp_t;
    mpfr_get_emax_max() -> mpfr_exp_t;
    mpfr_check_range(x: mpfr_ptr, t: c_int, rnd: mpfr_rnd_t) -> c_int;
    mpfr_subnormalize(x: mpfr_ptr, t: c_int, rnd: mpfr_rnd_t) -> c_int;
    mpfr_clear_underflow();
    mpfr_clear_overflow();
    mpfr_clear_divby0();
    mpfr_clear_nanflag();
    mpfr_clear_inexflag();
    mpfr_clear_erangeflag();
    mpfr_set_underflow();
    mpfr_set_overflow();
    mpfr_set_divby0();
    mpfr_set_nanflag();
    mpfr_set_inexflag();
    mpfr_set_erangeflag();
    mpfr_clear_flags();
    mpfr_underflow_p() -> c_int;
    mpfr_overflow_p() -> c_int;
    mpfr_divby0_p() -> c_int;
    mpfr_nanflag_p() -> c_int;
    mpfr_inexflag_p() -> c_int;
    mpfr_erangeflag_p() -> c_int;

    // Compatibility with MPF
    mpfr_set_prec_raw(x: mpfr_ptr, prec: mpfr_prec_t);
    mpfr_eq(op1: mpfr_srcptr, op2: mpfr_srcptr, op3: c_ulong) -> c_int;
    mpfr_reldiff(rop: mpfr_ptr,
                 op1: mpfr_srcptr,
                 op2: mpfr_srcptr,
                 rnd: mpfr_rnd_t);
    mpfr_mul_2exp(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: mpfr_rnd_t)
                  -> c_int;
    mpfr_div_2exp(rop: mpfr_ptr,
                  op1: mpfr_srcptr,
                  op2: c_ulong,
                  rnd: mpfr_rnd_t)
                  -> c_int;

    // Custom Interface
    mpfr_custom_get_size(prec: mpfr_prec_t) -> usize;
    mpfr_custom_init(significand: *mut c_void, prec: mpfr_prec_t);
    mpfr_custom_init_set(x: mpfr_ptr,
                         kind: c_int,
                         exp: mpfr_exp_t,
                         prec: mpfr_prec_t,
                         significand: *mut c_void);
    mpfr_custom_get_kind(x: mpfr_srcptr) -> c_int;
    mpfr_custom_get_significand(x: mpfr_srcptr) -> *mut c_void;
    mpfr_custom_get_exp(x: mpfr_srcptr) -> mpfr_exp_t;
    mpfr_custom_move(x: mpfr_ptr, new_position: *mut c_void);
}
