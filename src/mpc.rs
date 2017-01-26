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
    { $($c:tt $name:ident
        ($($par:ident: $ty:ty),* $(; $dots:tt)*) $(-> $ret:ty)*;
    )*
    } => {
        $(
            #[link(name = "mpc", kind = "static")]
            extern "C" {
                #[link_name = $c]
                pub fn $name($($par: $ty),* $(, $dots)*) $(-> $ret)*;
            }
        )*
    };
}

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

c_fn! {
    // Initialization Functions
    "mpc_init2" init2(z: mpc_ptr, prec: mpfr::prec_t);
    "mpc_init3" init3(z: mpc_ptr, prec_r: mpfr::prec_t, prec_i: mpfr::prec_t);
    "mpc_clear" clear(z: mpc_ptr);
    "mpc_set_prec" set_prec(x: mpc_ptr, prec: mpfr::prec_t);
    "mpc_get_prec" get_prec(x: mpc_srcptr) -> mpfr::prec_t;
    "mpc_get_prec2" get_prec2(pr: *mut mpfr::prec_t,
                              pi: *mut mpfr::prec_t,
                              x: mpc_srcptr);

    // Assignment Functions
    "mpc_set" set(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_set_ui" set_ui(rop: mpc_ptr, op: c_ulong, rnd: rnd_t) -> c_int;
    "mpc_set_si" set_si(rop: mpc_ptr, op: c_long, rnd: rnd_t) -> c_int;
    "mpc_set_d" set_d(rop: mpc_ptr, op: f64, rnd: rnd_t) -> c_int;
    "mpc_set_ld" set_ld(rop: mpc_ptr, op: f64, rnd: rnd_t) -> c_int;
    "mpc_set_z" set_z(rop: mpc_ptr, op: mpz_srcptr, rnd: rnd_t) -> c_int;
    "mpc_set_q" set_q(rop: mpc_ptr, op: mpq_srcptr, rnd: rnd_t) -> c_int;
    "mpc_set_f" set_f(rop: mpc_ptr, op: mpf_srcptr, rnd: rnd_t) -> c_int;
    "mpc_set_fr" set_fr(rop: mpc_ptr, op: mpfr_srcptr, rnd: rnd_t) -> c_int;
    "mpc_set_ui_ui" set_ui_ui(rop: mpc_ptr,
                              op1: c_ulong,
                              op2: c_ulong,
                              rnd: rnd_t)
                              -> c_int;
    "mpc_set_si_si" set_si_si(rop: mpc_ptr,
                              op1: c_long,
                              op2: c_long,
                              rnd: rnd_t)
                              -> c_int;
    "mpc_set_d_d" set_d_d(rop: mpc_ptr,
                          op1: f64,
                          op2: f64,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_set_ld_ld" set_ld_ld(rop: mpc_ptr,
                              op1: f64,
                              op2: f64,
                              rnd: rnd_t)
                              -> c_int;
    "mpc_set_z_z" set_z_z(rop: mpc_ptr,
                          op1: mpz_srcptr,
                          op2: mpz_srcptr,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_set_q_q" set_q_q(rop: mpc_ptr,
                          op1: mpq_srcptr,
                          op2: mpq_srcptr,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_set_f_f" set_f_f(rop: mpc_ptr,
                          op1: mpf_srcptr,
                          op2: mpf_srcptr,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_set_fr_fr" set_fr_fr(rop: mpc_ptr,
                              op1: mpfr_srcptr,
                              op2: mpfr_srcptr,
                              rnd: rnd_t)
                              -> c_int;
    "mpc_set_nan" set_nan(rop: mpc_ptr);
    "mpc_swap" swap(op1: mpc_ptr, op2: mpc_ptr);

    // String Input and Output
    "mpc_strtoc" strtoc(rop: mpc_ptr,
                        nptr: *const c_char,
                        endptr: *mut *mut c_char,
                        base: c_int,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_set_str" set_str(rop: mpc_ptr,
                          s: *const c_char,
                          base: c_int,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_get_str" get_str(b: c_int,
                          n: usize,
                          op: mpc_srcptr,
                          rnd: rnd_t)
                          -> *mut c_char;
    "mpc_free_str" free_str(rop: *mut c_char);

    // Comparison Functions
    "mpc_cmp" cmp(op1: mpc_srcptr, op2: mpc_srcptr) -> c_int;
    "mpc_cmp_si_si" cmp_si_si(op1: mpc_srcptr,
                              op2r: c_long,
                              op2i: c_long)
                              -> c_int;
}
#[inline]
pub unsafe fn cmp_si(op1: mpc_srcptr, op2: c_long) -> c_int {
    cmp_si_si(op1, op2, 0)
}
c_fn! {

    // Projection and Decomposing Functions
    "mpc_real" real(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    "mpc_imag" imag(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
}
#[inline]
pub unsafe fn realref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).re) as mpfr_ptr
}
#[inline]
pub unsafe fn imagref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).im) as mpfr_ptr
}
c_fn! {
    "mpc_arg" arg(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    "mpc_proj" proj(rop: mpc_ptr, arg2: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Basic Arithmetic Functions
    "mpc_add" add(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_add_ui" add_ui(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_ulong,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_add_fr" add_fr(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: mpfr_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_sub" sub(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_sub_fr" sub_fr(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: mpfr_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_fr_sub" fr_sub(rop: mpc_ptr,
                        op1: mpfr_srcptr,
                        op2: mpc_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_sub_ui" sub_ui(rop: mpc_ptr,
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
c_fn! {
    "mpc_ui_ui_sub" ui_ui_sub(rop: mpc_ptr,
                              re1: c_ulong,
                              im1: c_ulong,
                              op2: mpc_srcptr,
                              rnd: rnd_t)
                              -> c_int;
    "mpc_neg" neg(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_mul" mul(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_mul_ui" mul_ui(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_ulong,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_mul_si" mul_si(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_long,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_mul_fr" mul_fr(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: mpfr_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_mul_i" mul_i(rop: mpc_ptr,
                      op: mpc_srcptr,
                      sgn: c_int,
                      rnd: rnd_t)
                      -> c_int;
    "mpc_sqr" sqr(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_fma" fma(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  op3: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_div" div(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_div_ui" div_ui(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_ulong,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_div_fr" div_fr(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: mpfr_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_ui_div" ui_div(rop: mpc_ptr,
                        op1: c_ulong,
                        op2: mpc_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_fr_div" fr_div(rop: mpc_ptr,
                        op1: mpfr_srcptr,
                        op2: mpc_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_conj" conj(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_abs" abs(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    "mpc_norm" norm(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr::rnd_t) -> c_int;
    "mpc_mul_2ui" mul_2ui(rop: mpc_ptr,
                          op1: mpc_srcptr,
                          op2: c_ulong,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_mul_2si" mul_2si(rop: mpc_ptr,
                          op1: mpc_srcptr,
                          op2: c_long,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_div_2ui" div_2ui(rop: mpc_ptr,
                          op1: mpc_srcptr,
                          op2: c_ulong,
                          rnd: rnd_t)
                          -> c_int;
    "mpc_div_2si" div_2si(rop: mpc_ptr,
                          op1: mpc_srcptr,
                          op2: c_long,
                          rnd: rnd_t)
                          -> c_int;

    // Power Functions and Logarithms
    "mpc_sqrt" sqrt(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_pow" pow(rop: mpc_ptr,
                  op1: mpc_srcptr,
                  op2: mpc_srcptr,
                  rnd: rnd_t)
                  -> c_int;
    "mpc_pow_d" pow_d(rop: mpc_ptr,
                      op1: mpc_srcptr,
                      op2: f64,
                      rnd: rnd_t)
                      -> c_int;
    "mpc_pow_ld" pow_ld(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: f64,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_pow_si" pow_si(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_long,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_pow_ui" pow_ui(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: c_ulong,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_pow_z" pow_z(rop: mpc_ptr,
                      op1: mpc_srcptr,
                      op2: mpz_srcptr,
                      rnd: rnd_t)
                      -> c_int;
    "mpc_pow_fr" pow_fr(rop: mpc_ptr,
                        op1: mpc_srcptr,
                        op2: mpfr_srcptr,
                        rnd: rnd_t)
                        -> c_int;
    "mpc_exp" exp(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_log" log(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_log10" log10(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Trigonometric Functions
    "mpc_sin" sin(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_cos" cos(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_sin_cos" sin_cos(rop_sin: mpc_ptr,
                          rop_cos: mpc_ptr,
                          op: mpc_srcptr,
                          rnd_sin: rnd_t,
                          rnd_cos: rnd_t)
                          -> c_int;
    "mpc_tan" tan(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_sinh" sinh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_cosh" cosh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_tanh" tanh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_asin" asin(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_acos" acos(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_atan" atan(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_asinh" asinh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_acosh" acosh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;
    "mpc_atanh" atanh(rop: mpc_ptr, op: mpc_srcptr, rnd: rnd_t) -> c_int;

    // Miscellaneous Functions
    "mpc_urandom" urandom(rop: mpc_ptr, state: randstate_ptr) -> c_int;
    "mpc_get_version" get_version() -> *const c_char;
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
