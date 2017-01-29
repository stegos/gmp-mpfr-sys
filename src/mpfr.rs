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

c_fn! {
    // Initialization Functions
    "mpfr_init2" init2(x: mpfr_ptr, prec: prec_t);
    "mpfr_inits2" inits2(prec: prec_t, x: mpfr_ptr; ...);
    "mpfr_clear" clear(x: mpfr_ptr);
    "mpfr_clears" clears(x: mpfr_ptr; ...);
    "mpfr_init" init(x: mpfr_ptr);
    "mpfr_inits" inits(x: mpfr_ptr; ...);
    "mpfr_set_default_prec" set_default_prec(prec: prec_t);
    "mpfr_get_default_prec" get_default_prec() -> prec_t;
    "mpfr_set_prec" set_prec(x: mpfr_ptr, prec: prec_t);
    "mpfr_get_prec" get_prec(x: mpfr_srcptr) -> prec_t;

    // Assignment Functions
    "mpfr_set" set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_set_ui" set_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    "mpfr_set_si" set_si(rop: mpfr_ptr, op: c_long, rnd: rnd_t) -> c_int;
    "mpfr_set_flt" set_flt(rop: mpfr_ptr, op: f32, rnd: rnd_t) -> c_int;
    "mpfr_set_d" set_d(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    "mpfr_set_ld" set_ld(rop: mpfr_ptr, op: f64, rnd: rnd_t) -> c_int;
    "mpfr_set_z" set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_set_q" set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_set_f" set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_set_ui_2exp" set_ui_2exp(rop: mpfr_ptr,
                                   op: c_ulong,
                                   e: exp_t,
                                   rnd: rnd_t)
                                   -> c_int;
    "mpfr_set_si_2exp" set_si_2exp(rop: mpfr_ptr,
                                   op: c_long,
                                   e: exp_t,
                                   rnd: rnd_t)
                                   -> c_int;
    "mpfr_set_z_2exp" set_z_2exp(rop: mpfr_ptr,
                                 op: mpz_srcptr,
                                 e: exp_t,
                                 rnd: rnd_t)
                                 -> c_int;
    "mpfr_set_str" set_str(rop: mpfr_ptr,
                           s: *const c_char,
                           base: c_int,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_strtofr" strtofr(rop: mpfr_ptr,
                           nptr: *const c_char,
                           endptr: *mut *mut c_char,
                           base: c_int,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_set_nan" set_nan(x: mpfr_ptr);
    "mpfr_set_inf" set_inf(x: mpfr_ptr, sign: c_int);
    "mpfr_set_zero" set_zero(x: mpfr_ptr, sign: c_int);
    "mpfr_swap" swap(x: mpfr_ptr, y: mpfr_ptr);

    // Combined Initialization and Assignment Functions
}
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
c_fn! {
    "mpfr_init_set_str" init_set_str(x: mpfr_ptr,
                                     s: *const c_char,
                                     base: c_int,
                                     rnd: rnd_t)
                                     -> c_int;

    // Conversion Functions
    "mpfr_get_flt" get_flt(op: mpfr_srcptr, rnd: rnd_t) -> f32;
    "mpfr_get_d" get_d(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    "mpfr_get_ld" get_ld(op: mpfr_srcptr, rnd: rnd_t) -> f64;
    "mpfr_get_si" get_si(op: mpfr_srcptr, rnd: rnd_t) -> c_long;
    "mpfr_get_ui" get_ui(op: mpfr_srcptr, rnd: rnd_t) -> c_ulong;
    "mpfr_get_d_2exp" get_d_2exp(exp: *mut c_long,
                                 op: mpfr_srcptr,
                                 rnd: rnd_t)
                                 -> f64;
    "mpfr_get_ld_2exp" get_ld_2exp(exp: *mut c_long,
                                   op: mpfr_srcptr,
                                   rnd: rnd_t)
                                   -> f64;
    "mpfr_frexp" frexp(exp: *mut exp_t,
                       y: mpfr_ptr,
                       x: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_get_z_2exp" get_z_2exp(rop: mpz_ptr, op: mpfr_srcptr) -> exp_t;
    "mpfr_get_z" get_z(z: mpz_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_get_f" get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_get_str" get_str(str: *mut c_char,
                           expptr: *mut exp_t,
                           b: c_int,
                           n: usize,
                           op: mpfr_srcptr,
                           rnd: rnd_t)
                           -> *mut c_char;
    "mpfr_free_str" free_str(str: *mut c_char);
    "mpfr_fits_ulong_p" fits_ulong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_slong_p" fits_slong_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_uint_p" fits_uint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_sint_p" fits_sint_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_ushort_p" fits_ushort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_sshort_p" fits_sshort_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_uintmax_p" fits_uintmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fits_intmax_p" fits_intmax_p(op: mpfr_srcptr, rnd: rnd_t) -> c_int;

    // Basic Arithmetic Functions
    "mpfr_add" add(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_add_ui" add_ui(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_ulong,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_add_si" add_si(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_long,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_add_d" add_d(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: f64,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_add_z" add_z(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpz_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_add_q" add_q(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpq_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sub" sub(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_ui_sub" ui_sub(rop: mpfr_ptr,
                         op1: c_ulong,
                         op2: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_sub_ui" sub_ui(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_ulong,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_si_sub" si_sub(rop: mpfr_ptr,
                         op1: c_long,
                         op2: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_sub_si" sub_si(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_long,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_d_sub" d_sub(rop: mpfr_ptr,
                       op1: f64,
                       op2: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sub_d" sub_d(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: f64,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_z_sub" z_sub(rop: mpfr_ptr,
                       op1: mpz_srcptr,
                       op2: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sub_z" sub_z(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpz_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sub_q" sub_q(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpq_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_mul" mul(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_mul_ui" mul_ui(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_ulong,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_mul_si" mul_si(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_long,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_mul_d" mul_d(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: f64,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_mul_z" mul_z(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpz_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_mul_q" mul_q(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpq_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sqr" sqr(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_div" div(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_ui_div" ui_div(rop: mpfr_ptr,
                         op1: c_ulong,
                         op2: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_div_ui" div_ui(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_ulong,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_si_div" si_div(rop: mpfr_ptr,
                         op1: c_long,
                         op2: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_div_si" div_si(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_long,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_d_div" d_div(rop: mpfr_ptr,
                       op1: f64,
                       op2: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_div_d" div_d(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: f64,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_div_z" div_z(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpz_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_div_q" div_q(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpq_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_sqrt" sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_sqrt_ui" sqrt_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    "mpfr_rec_sqrt" rec_sqrt(rop: mpfr_ptr,
                             op: mpfr_srcptr,
                             rnd: rnd_t)
                             -> c_int;
    "mpfr_cbrt" cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_root" root(rop: mpfr_ptr,
                     op: mpfr_srcptr,
                     k: c_ulong,
                     rnd: rnd_t)
                     -> c_int;
    "mpfr_pow" pow(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_pow_ui" pow_ui(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_ulong,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_pow_si" pow_si(rop: mpfr_ptr,
                         op1: mpfr_srcptr,
                         op2: c_long,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_pow_z" pow_z(rop: mpfr_ptr,
                       op1: mpfr_srcptr,
                       op2: mpz_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_ui_pow_ui" ui_pow_ui(rop: mpfr_ptr,
                               op1: c_ulong,
                               op2: c_ulong,
                               rnd: rnd_t)
                               -> c_int;
    "mpfr_ui_pow" ui_pow(rop: mpfr_ptr,
                         op1: c_ulong,
                         op2: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_neg" neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_abs" abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_dim" dim(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_mul_2ui" mul_2ui(rop: mpfr_ptr,
                           op1: mpfr_srcptr,
                           op2: c_ulong,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_mul_2si" mul_2si(rop: mpfr_ptr,
                           op1: mpfr_srcptr,
                           op2: c_long,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_div_2ui" div_2ui(rop: mpfr_ptr,
                           op1: mpfr_srcptr,
                           op2: c_ulong,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_div_2si" div_2si(rop: mpfr_ptr,
                           op1: mpfr_srcptr,
                           op2: c_long,
                           rnd: rnd_t)
                           -> c_int;

    // Comparison Functions
    "mpfr_cmp" cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_cmp_ui" cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;
    "mpfr_cmp_si" cmp_si(op1: mpfr_srcptr, op2: c_long) -> c_int;
    "mpfr_cmp_d" cmp_d(op1: mpfr_srcptr, op2: f64) -> c_int;
    "mpfr_cmp_ld" cmp_ld(op1: mpfr_srcptr, op2: f64) -> c_int;
    "mpfr_cmp_z" cmp_z(op1: mpfr_srcptr, op2: mpz_srcptr) -> c_int;
    "mpfr_cmp_q" cmp_q(op1: mpfr_srcptr, op2: mpq_srcptr) -> c_int;
    "mpfr_cmp_f" cmp_f(op1: mpfr_srcptr, op2: mpf_srcptr) -> c_int;
    "mpfr_cmp_ui_2exp" cmp_ui_2exp(op1: mpfr_srcptr,
                                   op2: c_ulong,
                                   e: exp_t)
                                   -> c_int;
    "mpfr_cmp_si_2exp" cmp_si_2exp(op1: mpfr_srcptr,
                                   op2: c_long,
                                   e: exp_t)
                                   -> c_int;
    "mpfr_cmpabs" cmpabs(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_nan_p" nan_p(op: mpfr_srcptr) -> c_int;
    "mpfr_inf_p" inf_p(op: mpfr_srcptr) -> c_int;
    "mpfr_number_p" number_p(op: mpfr_srcptr) -> c_int;
    "mpfr_zero_p" zero_p(op: mpfr_srcptr) -> c_int;
    "mpfr_regular_p" regular_p(op: mpfr_srcptr) -> c_int;
    "mpfr_sgn" sgn(op: mpfr_srcptr) -> c_int;
    "mpfr_greater_p" greater_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_greaterequal_p" greaterequal_p(op1: mpfr_srcptr,
                                         op2: mpfr_srcptr)
                                         -> c_int;
    "mpfr_less_p" less_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_lessequal_p" lessequal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_equal_p" equal_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    "mpfr_lessgreater_p" lessgreater_p(op1: mpfr_srcptr,
                                       op2: mpfr_srcptr)
                                       -> c_int;
    "mpfr_unordered_p" unordered_p(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;

    // Special Functions
    "mpfr_log" log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_log2" log2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_log10" log10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_exp" exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_exp2" exp2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_exp10" exp10(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_cos" cos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_sin" sin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_tan" tan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_sin_cos" sin_cos(sop: mpfr_ptr,
                           cop: mpfr_ptr,
                           op: mpfr_srcptr,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_sec" sec(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_csc" csc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_cot" cot(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_acos" acos(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_asin" asin(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_atan" atan(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_atan2" atan2(rop: mpfr_ptr,
                       y: mpfr_srcptr,
                       x: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_cosh" cosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_sinh" sinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_tanh" tanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_sinh_cosh" sinh_cosh(sop: mpfr_ptr,
                               cop: mpfr_ptr,
                               op: mpfr_srcptr,
                               rnd: rnd_t)
                               -> c_int;
    "mpfr_sech" sech(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_csch" csch(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_coth" coth(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_acosh" acosh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_asinh" asinh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_atanh" atanh(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_fac_ui" fac_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    "mpfr_log1p" log1p(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_expm1" expm1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_eint" eint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_li2" li2(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_gamma" gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_lngamma" lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_lgamma" lgamma(rop: mpfr_ptr,
                         signp: *mut c_int,
                         op: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_digamma" digamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_zeta" zeta(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_zeta_ui" zeta_ui(rop: mpfr_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    "mpfr_erf" erf(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_erfc" erfc(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_j0" j0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_j1" j1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_jn" jn(rop: mpfr_ptr,
                 n: c_long,
                 op: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    "mpfr_y0" y0(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_y1" y1(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_yn" yn(rop: mpfr_ptr,
                 n: c_long,
                 op: mpfr_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    "mpfr_fma" fma(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   op3: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_fms" fms(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   op3: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_agm" agm(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_hypot" hypot(rop: mpfr_ptr,
                       x: mpfr_srcptr,
                       y: mpfr_srcptr,
                       rnd: rnd_t)
                       -> c_int;
    "mpfr_ai" ai(rop: mpfr_ptr, x: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_const_log2" const_log2(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    "mpfr_const_pi" const_pi(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    "mpfr_const_euler" const_euler(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    "mpfr_const_catalan" const_catalan(rop: mpfr_ptr, rnd: rnd_t) -> c_int;
    "mpfr_free_cache" free_cache();
    "mpfr_sum" sum(rop: mpfr_ptr,
                   tab: *mut mpfr_ptr,
                   n: c_ulong,
                   rnd: rnd_t)
                   -> c_int;

    // Formatted Output Functions
    "mpfr_printf" printf(template: *const c_char; ...) -> c_int;
    "mpfr_sprintf" sprintf(buf: *mut c_char,
                           template: *const c_char; ...)
                           -> c_int;
    "mpfr_snprintf" snprintf(buf: *mut c_char,
                             n: usize,
                             template: *const c_char;
                             ...)
                             -> c_int;
    "mpfr_asprintf" asprintf(str: *mut *mut c_char,
                             template: *const c_char;
                             ...)
                             -> c_int;

    // Integer and Remainder Related Functions
    "mpfr_rint" rint(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_ceil" ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    "mpfr_floor" floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    "mpfr_round" round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    "mpfr_trunc" trunc(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    "mpfr_rint_ceil" rint_ceil(rop: mpfr_ptr,
                               op: mpfr_srcptr,
                               rnd: rnd_t)
                               -> c_int;
    "mpfr_rint_floor" rint_floor(rop: mpfr_ptr,
                                 op: mpfr_srcptr,
                                 rnd: rnd_t)
                                 -> c_int;
    "mpfr_rint_round" rint_round(rop: mpfr_ptr,
                                 op: mpfr_srcptr,
                                 rnd: rnd_t)
                                 -> c_int;
    "mpfr_rint_trunc" rint_trunc(rop: mpfr_ptr,
                                 op: mpfr_srcptr,
                                 rnd: rnd_t)
                                 -> c_int;
    "mpfr_frac" frac(rop: mpfr_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpfr_modf" modf(iop: mpfr_ptr,
                     fop: mpfr_ptr,
                     op: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    "mpfr_fmod" fmod(r: mpfr_ptr,
                     x: mpfr_srcptr,
                     y: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    "mpfr_remainder" remainder(r: mpfr_ptr,
                               x: mpfr_srcptr,
                               y: mpfr_srcptr,
                               rnd: rnd_t)
                               -> c_int;
    "mpfr_remquo" remquo(r: mpfr_ptr,
                         q: *mut c_long,
                         x: mpfr_srcptr,
                         y: mpfr_srcptr,
                         rnd: rnd_t)
                         -> c_int;
    "mpfr_integer_p" integer_p(op: mpfr_srcptr) -> c_int;

    // Rounding Related Functions
    "mpfr_set_default_rounding_mode" set_default_rounding_mode(rnd: rnd_t);
    "mpfr_get_default_rounding_mode" get_default_rounding_mode() -> rnd_t;
    "mpfr_prec_round" prec_round(x: mpfr_ptr,
                                 prec: prec_t,
                                 rnd: rnd_t)
                                 -> c_int;
    "mpfr_can_round" can_round(b: mpfr_srcptr,
                               err: exp_t,
                               rnd1: rnd_t,
                               rnd2: rnd_t,
                               prec: prec_t)
                               -> c_int;
    "mpfr_min_prec" min_prec(x: mpfr_srcptr) -> prec_t;
    "mpfr_print_rnd_mode" print_rnd_mode(rnd: rnd_t) -> *const c_char;

    // Miscellaneous Functions
    "mpfr_nexttoward" nexttoward(x: mpfr_ptr, y: mpfr_srcptr);
    "mpfr_nextabove" nextabove(x: mpfr_ptr);
    "mpfr_nextbelow" nextbelow(x: mpfr_ptr);
    "mpfr_min" min(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_max" max(rop: mpfr_ptr,
                   op1: mpfr_srcptr,
                   op2: mpfr_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    "mpfr_urandomb" urandomb(rop: mpfr_ptr, state: randstate_ptr) -> c_int;
    "mpfr_urandom" urandom(rop: mpfr_ptr,
                           state: randstate_ptr,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_grandom" grandom(rop1: mpfr_ptr,
                           rop2: mpfr_ptr,
                           state: randstate_ptr,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_get_exp" get_exp(x: mpfr_srcptr) -> exp_t;
    "mpfr_set_exp" set_exp(x: mpfr_ptr, e: exp_t) -> c_int;
    "mpfr_signbit" signbit(op: mpfr_srcptr) -> c_int;
    "mpfr_setsign" setsign(rop: mpfr_ptr,
                           op: mpfr_srcptr,
                           s: c_int,
                           rnd: rnd_t)
                           -> c_int;
    "mpfr_copysign" copysign(rop: mpfr_ptr,
                             op1: mpfr_srcptr,
                             op2: mpfr_srcptr,
                             rnd: rnd_t)
                             -> c_int;
    "mpfr_get_version" get_version() -> *const c_char;
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
c_fn! {
    "mpfr_get_patches" get_patches() -> *const c_char;
    "mpfr_buildopt_tls_p" buildopt_tls_p() -> c_int;
    "mpfr_buildopt_decimal_p" buildopt_decimal_p() -> c_int;
    "mpfr_buildopt_gmpinternals_p" buildopt_gmpinternals_p() -> c_int;
    "mpfr_buildopt_tune_case" buildopt_tune_case() -> *const c_char;

    // Exception Related Functions
    "mpfr_get_emin" get_emin() -> exp_t;
    "mpfr_get_emax" get_emax() -> exp_t;
    "mpfr_set_emin" set_emin(exp: exp_t) -> c_int;
    "mpfr_set_emax" set_emax(exp: exp_t) -> c_int;
    "mpfr_get_emin_min" get_emin_min() -> exp_t;
    "mpfr_get_emin_max" get_emin_max() -> exp_t;
    "mpfr_get_emax_min" get_emax_min() -> exp_t;
    "mpfr_get_emax_max" get_emax_max() -> exp_t;
    "mpfr_check_range" check_range(x: mpfr_ptr, t: c_int, rnd: rnd_t) -> c_int;
    "mpfr_subnormalize" subnormalize(x: mpfr_ptr,
                                     t: c_int,
                                     rnd: rnd_t)
                                     -> c_int;
    "mpfr_clear_underflow" clear_underflow();
    "mpfr_clear_overflow" clear_overflow();
    "mpfr_clear_divby0" clear_divby0();
    "mpfr_clear_nanflag" clear_nanflag();
    "mpfr_clear_inexflag" clear_inexflag();
    "mpfr_clear_erangeflag" clear_erangeflag();
    "mpfr_set_underflow" set_underflow();
    "mpfr_set_overflow" set_overflow();
    "mpfr_set_divby0" set_divby0();
    "mpfr_set_nanflag" set_nanflag();
    "mpfr_set_inexflag" set_inexflag();
    "mpfr_set_erangeflag" set_erangeflag();
    "mpfr_clear_flags" clear_flags();
    "mpfr_underflow_p" underflow_p() -> c_int;
    "mpfr_overflow_p" overflow_p() -> c_int;
    "mpfr_divby0_p" divby0_p() -> c_int;
    "mpfr_nanflag_p" nanflag_p() -> c_int;
    "mpfr_inexflag_p" inexflag_p() -> c_int;
    "mpfr_erangeflag_p" erangeflag_p() -> c_int;

    // Compatibility with MPF
    "mpfr_set_prec_raw" set_prec_raw(x: mpfr_ptr, prec: prec_t);
    "mpfr_eq" eq(op1: mpfr_srcptr, op2: mpfr_srcptr, op3: c_ulong) -> c_int;
    "mpfr_reldiff" reldiff(rop: mpfr_ptr,
                           op1: mpfr_srcptr,
                           op2: mpfr_srcptr,
                           rnd: rnd_t);
    "mpfr_mul_2exp" mul_2exp(rop: mpfr_ptr,
                             op1: mpfr_srcptr,
                             op2: c_ulong,
                             rnd: rnd_t)
                             -> c_int;
    "mpfr_div_2exp" div_2exp(rop: mpfr_ptr,
                             op1: mpfr_srcptr,
                             op2: c_ulong,
                             rnd: rnd_t)
                             -> c_int;

    // Custom Interface
    "mpfr_custom_get_size" custom_get_size(prec: prec_t) -> usize;
    "mpfr_custom_init" custom_init(significand: *mut c_void, prec: prec_t);
    "mpfr_custom_init_set" custom_init_set(x: mpfr_ptr,
                                           kind: c_int,
                                           exp: exp_t,
                                           prec: prec_t,
                                           significand: *mut c_void);
    "mpfr_custom_get_kind" custom_get_kind(x: mpfr_srcptr) -> c_int;
    "mpfr_custom_get_significand" custom_get_significand(x: mpfr_srcptr)
                                                         -> *mut c_void;
    "mpfr_custom_get_exp" custom_get_exp(x: mpfr_srcptr) -> exp_t;
    "mpfr_custom_move" custom_move(x: mpfr_ptr, new_position: *mut c_void);
}
