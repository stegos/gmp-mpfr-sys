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
use mpfr;
use std::os::raw::{c_char, c_int, c_long, c_ulong};

#[inline]
pub fn INEX_POS(inex: c_int) -> c_int {
    match inex {
        neg if neg < 0 => 2,
        0 => 0,
        _ => 1,
    }
}
#[inline]
pub fn INEX_NEG(inex: c_int) -> c_int {
    match inex {
        2 => -1,
        0 => 0,
        _ => 1,
    }
}
#[inline]
pub fn INEX(inex_re: c_int, inex_im: c_int) -> c_int {
    INEX_POS(inex_re) | (INEX_POS(inex_im) << 2)
}
#[inline]
pub fn INEX_RE(inex: c_int) -> c_int {
    INEX_NEG((inex) & 3)
}
#[inline]
pub fn INEX_IM(inex: c_int) -> c_int {
    INEX_NEG((inex) >> 2)
}
#[inline]
pub fn INEX12(inex1: c_int, inex2: c_int) -> c_int {
    inex1 | (inex2 << 4)
}
#[inline]
pub fn INEX1(inex: c_int) -> c_int {
    inex & 15
}
#[inline]
pub fn INEX2(inex: c_int) -> c_int {
    inex >> 4
}

pub type rnd_t = c_int;

#[inline]
pub fn RND(r1: c_int, r2: c_int) -> c_int {
    r1 + (r2 << 4)
}
#[inline]
pub fn RND_RE(x: c_int) -> mpfr::rnd_t {
    as_mpfr_rnd_t(x & 0x0f)
}
#[inline]
pub fn RND_IM(x: c_int) -> mpfr::rnd_t {
    as_mpfr_rnd_t(x >> 4)
}

#[inline]
fn as_mpfr_rnd_t(x: c_int) -> mpfr::rnd_t {
    match x {
        0 => mpfr::rnd_t::RNDN,
        1 => mpfr::rnd_t::RNDZ,
        2 => mpfr::rnd_t::RNDU,
        3 => mpfr::rnd_t::RNDD,
        4 => mpfr::rnd_t::RNDA,
        5 => mpfr::rnd_t::RNDF,
        _ => mpfr::rnd_t::RNDNA,
    }
}

const RNDN: c_int = mpfr::rnd_t::RNDN as c_int;
const RNDZ: c_int = mpfr::rnd_t::RNDZ as c_int;
const RNDU: c_int = mpfr::rnd_t::RNDU as c_int;
const RNDD: c_int = mpfr::rnd_t::RNDD as c_int;

pub const RNDNN: c_int = RNDN + (RNDN << 4);
pub const RNDNZ: c_int = RNDN + (RNDZ << 4);
pub const RNDNU: c_int = RNDN + (RNDU << 4);
pub const RNDND: c_int = RNDN + (RNDD << 4);

pub const RNDZN: c_int = RNDZ + (RNDN << 4);
pub const RNDZZ: c_int = RNDZ + (RNDZ << 4);
pub const RNDZU: c_int = RNDZ + (RNDU << 4);
pub const RNDZD: c_int = RNDZ + (RNDD << 4);

pub const RNDUN: c_int = RNDU + (RNDN << 4);
pub const RNDUZ: c_int = RNDU + (RNDZ << 4);
pub const RNDUU: c_int = RNDU + (RNDU << 4);
pub const RNDUD: c_int = RNDU + (RNDD << 4);

pub const RNDDN: c_int = RNDD + (RNDN << 4);
pub const RNDDZ: c_int = RNDD + (RNDZ << 4);
pub const RNDDU: c_int = RNDD + (RNDU << 4);
pub const RNDDD: c_int = RNDD + (RNDD << 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct mpc_t {
    pub re: mpfr::mpfr_t,
    pub im: mpfr::mpfr_t,
}

// Types for function declarations in this file.

type mpz_srcptr = *const gmp::mpz_t;
type mpq_srcptr = *const gmp::mpq_t;
type mpf_srcptr = *const gmp::mpf_t;
type randstate_ptr = *mut gmp::randstate_t;
type mpfr_srcptr = *const mpfr::mpfr_t;
type mpfr_ptr = *mut mpfr::mpfr_t;
type mpc_ptr = *mut mpc_t;
type mpc_srcptr = *const mpc_t;

extern "C" {
    // Initialization Functions

    #[link_name = "mpc_init2"]
    pub fn init2(z: mpc_ptr, prec: mpfr::prec_t);
    #[link_name = "mpc_init3"]
    pub fn init3(z: mpc_ptr, prec_r: mpfr::prec_t, prec_i: mpfr::prec_t);
    #[link_name = "mpc_clear"]
    pub fn clear(z: mpc_ptr);
    #[link_name = "mpc_set_prec"]
    pub fn set_prec(x: mpc_ptr, prec: mpfr::prec_t);
    #[link_name = "mpc_get_prec"]
    pub fn get_prec(x: mpc_srcptr) -> mpfr::prec_t;
    #[link_name = "mpc_get_prec2"]
    pub fn get_prec2(pr: *mut mpfr::prec_t,
                     pi: *mut mpfr::prec_t,
                     x: mpc_srcptr);

    // Assignment Functions

    #[link_name = "mpc_set"]
    pub fn set(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_ui"]
    pub fn set_ui(rop: mpc_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_si"]
    pub fn set_si(rop: mpc_ptr, op: c_long, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_d"]
    pub fn set_d(rop: mpc_ptr, op: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_ld"]
    pub fn set_ld(rop: mpc_ptr, op: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_z"]
    pub fn set_z(rop: mpc_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_q"]
    pub fn set_q(rop: mpc_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_f"]
    pub fn set_f(rop: mpc_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_fr"]
    pub fn set_fr(rop: mpc_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_ui_ui"]
    pub fn set_ui_ui(rop: mpc_ptr,
                     op1: c_ulong,
                     op2: c_ulong,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpc_set_si_si"]
    pub fn set_si_si(rop: mpc_ptr,
                     op1: c_long,
                     op2: c_long,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpc_set_d_d"]
    pub fn set_d_d(rop: mpc_ptr, op1: f64, op2: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_ld_ld"]
    pub fn set_ld_ld(rop: mpc_ptr, op1: f64, op2: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_set_z_z"]
    pub fn set_z_z(rop: mpc_ptr,
                   op1: mpz_srcptr,
                   op2: mpz_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_set_q_q"]
    pub fn set_q_q(rop: mpc_ptr,
                   op1: mpq_srcptr,
                   op2: mpq_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_set_f_f"]
    pub fn set_f_f(rop: mpc_ptr,
                   op1: mpf_srcptr,
                   op2: mpf_srcptr,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_set_fr_fr"]
    pub fn set_fr_fr(rop: mpc_ptr,
                     op1: mpfr_srcptr,
                     op2: mpfr_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpc_set_nan"]
    pub fn set_nan(rop: mpc_ptr);
    #[link_name = "mpc_swap"]
    pub fn swap(op1: mpc_ptr, op2: mpc_ptr);

    // String Input and Output

    #[link_name = "mpc_strtoc"]
    pub fn strtoc(rop: mpc_ptr,
                  nptr: *const c_char,
                  endptr: *mut *mut c_char,
                  base: c_int,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_set_str"]
    pub fn set_str(rop: mpc_ptr,
                   s: *const c_char,
                   base: c_int,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_get_str"]
    pub fn get_str(b: c_int,
                   n: usize,
                   op: mpc_srcptr,
                   rnd: rnd_t)
                   -> *mut c_char;
    #[link_name = "mpc_free_str"]
    pub fn free_str(rop: *mut c_char);

    // Comparison Functions

    #[link_name = "mpc_cmp"]
    pub fn cmp(op1: mpc_srcptr, op2: mpc_srcptr) -> c_int;
    #[link_name = "mpc_cmp_si_si"]
    pub fn cmp_si_si(op1: mpc_srcptr, op2r: c_long, op2i: c_long) -> c_int;
}
#[inline]
pub unsafe fn cmp_si(op1: mpc_srcptr, op2: c_long) -> c_int {
    cmp_si_si(op1, op2, 0)
}

extern "C" {
    // Projection and Decomposing Functions
    #[link_name = "mpc_real"]
    pub fn real(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    #[link_name = "mpc_imag"]
    pub fn imag(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
}
#[inline]
pub unsafe fn realref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).re) as mpfr_ptr
}
#[inline]
pub unsafe fn imagref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).im) as mpfr_ptr
}
extern "C" {
    #[link_name = "mpc_arg"]
    pub fn arg(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    #[link_name = "mpc_proj"]
    pub fn proj(rop: mpc_ptr, arg2: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Basic Arithmetic Functions

    #[link_name = "mpc_add"]
    pub fn add(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_add_ui"]
    pub fn add_ui(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_add_fr"]
    pub fn add_fr(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_sub"]
    pub fn sub(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_sub_fr"]
    pub fn sub_fr(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_fr_sub"]
    pub fn fr_sub(rop: mpc_ptr,
                  op1: mpfr_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_sub_ui"]
    pub fn sub_ui(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
}
#[inline]
pub unsafe fn ui_sub(rop: mpc_ptr,
                     op1: c_ulong,
                     op2: mpc_srcptr,
                     rnd: rnd_t)
                     -> c_int {
    ui_ui_sub(rop, op1, 0, op2, rnd)
}
extern "C" {
    #[link_name = "mpc_ui_ui_sub"]
    pub fn ui_ui_sub(rop: mpc_ptr,
                     re1: c_ulong,
                     im1: c_ulong,
                     op2: mpc_srcptr,
                     rnd: rnd_t)
                     -> c_int;
    #[link_name = "mpc_neg"]
    pub fn neg(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_mul"]
    pub fn mul(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_mul_ui"]
    pub fn mul_ui(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_mul_si"]
    pub fn mul_si(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_mul_fr"]
    pub fn mul_fr(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_mul_i"]
    pub fn mul_i(rop: mpc_ptr,
                 op: mpc_srcptr,
                 sgn: c_int,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpc_sqr"]
    pub fn sqr(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_fma"]
    pub fn fma(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               op3: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_div"]
    pub fn div(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_div_ui"]
    pub fn div_ui(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_div_fr"]
    pub fn div_fr(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_ui_div"]
    pub fn ui_div(rop: mpc_ptr,
                  op1: c_ulong,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_fr_div"]
    pub fn fr_div(rop: mpc_ptr,
                  op1: mpfr_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_conj"]
    pub fn conj(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_abs"]
    pub fn abs(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    #[link_name = "mpc_norm"]
    pub fn norm(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    #[link_name = "mpc_mul_2ui"]
    pub fn mul_2ui(rop: mpc_ptr,
                   op1: mpc_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_mul_2si"]
    pub fn mul_2si(rop: mpc_ptr,
                   op1: mpc_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_div_2ui"]
    pub fn div_2ui(rop: mpc_ptr,
                   op1: mpc_srcptr,
                   op2: c_ulong,
                   rnd: rnd_t)
                   -> c_int;
    #[link_name = "mpc_div_2si"]
    pub fn div_2si(rop: mpc_ptr,
                   op1: mpc_srcptr,
                   op2: c_long,
                   rnd: rnd_t)
                   -> c_int;

    // Power Functions and Logarithms

    #[link_name = "mpc_sqrt"]
    pub fn sqrt(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_pow"]
    pub fn pow(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpc_srcptr,
               rnd: rnd_t)
               -> c_int;
    #[link_name = "mpc_pow_d"]
    pub fn pow_d(rop: mpc_ptr, op1: mpc_srcptr, op2: f64, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_pow_ld"]
    pub fn pow_ld(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: f64,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_pow_si"]
    pub fn pow_si(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_long,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_pow_ui"]
    pub fn pow_ui(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: c_ulong,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_pow_z"]
    pub fn pow_z(rop: mpc_ptr,
                 op1: mpc_srcptr,
                 op2: mpz_srcptr,
                 rnd: rnd_t)
                 -> c_int;
    #[link_name = "mpc_pow_fr"]
    pub fn pow_fr(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpfr_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    #[link_name = "mpc_exp"]
    pub fn exp(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_log"]
    pub fn log(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_log10"]
    pub fn log10(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Trigonometric Functions

    #[link_name = "mpc_sin"]
    pub fn sin(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_cos"]
    pub fn cos(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_sin_cos"]
    pub fn sin_cos(rop_sin: mpc_ptr,
                   rop_cos: mpc_ptr,
                   op: mpc_srcptr,
                   rnd_sin: rnd_t,
                   rnd_cos: rnd_t)
                   -> c_int;
    #[link_name = "mpc_tan"]
    pub fn tan(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_sinh"]
    pub fn sinh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_cosh"]
    pub fn cosh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_tanh"]
    pub fn tanh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_asin"]
    pub fn asin(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_acos"]
    pub fn acos(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_atan"]
    pub fn atan(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_asinh"]
    pub fn asinh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_acosh"]
    pub fn acosh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    #[link_name = "mpc_atanh"]
    pub fn atanh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Miscellaneous Functions

    #[link_name = "mpc_urandom"]
    pub fn urandom(rop: mpc_ptr, state: randstate_ptr) -> c_int;
    #[link_name = "mpc_get_version"]
    pub fn get_version() -> *const c_char;
}
pub const VERSION: c_int = (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) |
                           VERSION_PATCHLEVEL;
pub const VERSION_MAJOR: c_int = 1;
pub const VERSION_MINOR: c_int = 0;
pub const VERSION_PATCHLEVEL: c_int = 3;
pub const VERSION_STRING: *const c_char = b"1.0.3\0" as *const u8 as
                                          *const c_char;
#[inline]
pub fn VERSION_NUM(major: c_int, minor: c_int, patchlevel: c_int) -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
