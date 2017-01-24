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
                #[link(name = "mpc", kind = "static")]
                extern "C" {
                    pub fn $c($($par: $ty),* $(, $dots)*) $(-> $ret)*;
                }
            )*
        )*
    };
}

use ::gmp::*;
use ::mpfr::*;
use std::os::raw::{c_char, c_int, c_long, c_ulong};

type mpz_srcptr = *const mpz_t;
type mpq_srcptr = *const mpq_t;
type mpf_srcptr = *const mpf_t;
type randstate_ptr = *mut gmp_randstate_t;
type mpfr_srcptr = *const mpfr_t;
type mpfr_ptr = *mut mpfr_t;

#[inline]
pub fn MPC_INEX_POS(inex: c_int) -> c_int {
    match inex {
        neg if neg < 0 => 2,
        0 => 0,
        _ => 1,
    }
}
#[inline]
pub fn MPC_INEX_NEG(inex: c_int) -> c_int {
    match inex {
        2 => -1,
        0 => 0,
        _ => 1,
    }
}
#[inline]
pub fn MPC_INEX(inex_re: c_int, inex_im: c_int) -> c_int {
    MPC_INEX_POS(inex_re) | (MPC_INEX_POS(inex_im) << 2)
}
#[inline]
pub fn MPC_INEX_RE(inex: c_int) -> c_int {
    MPC_INEX_NEG((inex) & 3)
}
#[inline]
pub fn MPC_INEX_IM(inex: c_int) -> c_int {
    MPC_INEX_NEG((inex) >> 2)
}
#[inline]
pub fn MPC_INEX12(inex1: c_int, inex2: c_int) -> c_int {
    inex1 | (inex2 << 4)
}
#[inline]
pub fn MPC_INEX1(inex: c_int) -> c_int {
    inex & 15
}
#[inline]
pub fn MPC_INEX2(inex: c_int) -> c_int {
    inex >> 4
}

pub type mpc_rnd_t = c_int;

#[inline]
pub fn MPC_RND(r1: c_int, r2: c_int) -> c_int {
    r1 + (r2 << 4)
}
#[inline]
pub fn MPC_RND_RE(x: c_int) -> mpfr_rnd_t {
    as_mpfr_rnd_t(x & 0x0f)
}
#[inline]
pub fn MPC_RND_IM(x: c_int) -> mpfr_rnd_t {
    as_mpfr_rnd_t(x >> 4)
}

#[inline]
fn as_mpfr_rnd_t(x: c_int) -> mpfr_rnd_t {
    match x {
        0 => mpfr_rnd_t::MPFR_RNDN,
        1 => mpfr_rnd_t::MPFR_RNDZ,
        2 => mpfr_rnd_t::MPFR_RNDU,
        3 => mpfr_rnd_t::MPFR_RNDD,
        4 => mpfr_rnd_t::MPFR_RNDA,
        5 => mpfr_rnd_t::MPFR_RNDF,
        _ => mpfr_rnd_t::MPFR_RNDNA,
    }
}

pub const MPC_RNDNN: c_int = GMP_RNDN as c_int + ((GMP_RNDN as c_int) << 4);
pub const MPC_RNDNZ: c_int = GMP_RNDN as c_int + ((GMP_RNDZ as c_int) << 4);
pub const MPC_RNDNU: c_int = GMP_RNDN as c_int + ((GMP_RNDU as c_int) << 4);
pub const MPC_RNDND: c_int = GMP_RNDN as c_int + ((GMP_RNDD as c_int) << 4);

pub const MPC_RNDZN: c_int = GMP_RNDZ as c_int + ((GMP_RNDN as c_int) << 4);
pub const MPC_RNDZZ: c_int = GMP_RNDZ as c_int + ((GMP_RNDZ as c_int) << 4);
pub const MPC_RNDZU: c_int = GMP_RNDZ as c_int + ((GMP_RNDU as c_int) << 4);
pub const MPC_RNDZD: c_int = GMP_RNDZ as c_int + ((GMP_RNDD as c_int) << 4);

pub const MPC_RNDUN: c_int = GMP_RNDU as c_int + ((GMP_RNDN as c_int) << 4);
pub const MPC_RNDUZ: c_int = GMP_RNDU as c_int + ((GMP_RNDZ as c_int) << 4);
pub const MPC_RNDUU: c_int = GMP_RNDU as c_int + ((GMP_RNDU as c_int) << 4);
pub const MPC_RNDUD: c_int = GMP_RNDU as c_int + ((GMP_RNDD as c_int) << 4);

pub const MPC_RNDDN: c_int = GMP_RNDD as c_int + ((GMP_RNDN as c_int) << 4);
pub const MPC_RNDDZ: c_int = GMP_RNDD as c_int + ((GMP_RNDZ as c_int) << 4);
pub const MPC_RNDDU: c_int = GMP_RNDD as c_int + ((GMP_RNDU as c_int) << 4);
pub const MPC_RNDDD: c_int = GMP_RNDD as c_int + ((GMP_RNDD as c_int) << 4);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct __mpc_struct {
    pub re: mpfr_t,
    pub im: mpfr_t,
}
pub type mpc_t = __mpc_struct;

type mpc_ptr = *mut __mpc_struct;
type mpc_srcptr = *const __mpc_struct;

c_fn! {
    // Initialization Functions
    mpc_init2(z: mpc_ptr, prec: mpfr_prec_t);
    mpc_init3(z: mpc_ptr, prec_r: mpfr_prec_t, prec_i: mpfr_prec_t);
    mpc_clear(z: mpc_ptr);
    mpc_set_prec(x: mpc_ptr, prec: mpfr_prec_t);
    mpc_get_prec(x: mpc_srcptr) -> mpfr_prec_t;
    mpc_get_prec2(pr: *mut mpfr_prec_t,
                  pi: *mut mpfr_prec_t,
                  x: mpc_srcptr);

    // Assignment Functions
    mpc_set(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_set_ui(rop: mpc_ptr, op: c_ulong, rnd: mpc_rnd_t) -> c_int;
    mpc_set_si(rop: mpc_ptr, op: c_long, rnd: mpc_rnd_t) -> c_int;
    mpc_set_d(rop: mpc_ptr, op: f64, rnd: mpc_rnd_t) -> c_int;
    mpc_set_ld(rop: mpc_ptr, op: f64, rnd: mpc_rnd_t) -> c_int;
    mpc_set_z(rop: mpc_ptr, op: mpz_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_set_q(rop: mpc_ptr, op: mpq_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_set_f(rop: mpc_ptr, op: mpf_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_set_fr(rop: mpc_ptr, op: mpfr_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_set_ui_ui(rop: mpc_ptr,
                  op1: c_ulong,
                  op2: c_ulong,
                  rnd: mpc_rnd_t)
                  -> c_int;
    mpc_set_si_si(rop: mpc_ptr,
                  op1: c_long,
                  op2: c_long,
                  rnd: mpc_rnd_t)
                  -> c_int;
    mpc_set_d_d(rop: mpc_ptr,
                op1: f64,
                op2: f64,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_set_ld_ld(rop: mpc_ptr,
                  op1: f64,
                  op2: f64,
                  rnd: mpc_rnd_t)
                  -> c_int;
    mpc_set_z_z(rop: mpc_ptr,
                op1: mpz_srcptr,
                op2: mpz_srcptr,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_set_q_q(rop: mpc_ptr,
                op1: mpq_srcptr,
                op2: mpq_srcptr,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_set_f_f(rop: mpc_ptr,
                op1: mpf_srcptr,
                op2: mpf_srcptr,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_set_fr_fr(rop: mpc_ptr,
                  op1: mpfr_srcptr,
                  op2: mpfr_srcptr,
                  rnd: mpc_rnd_t)
                  -> c_int;
    mpc_set_nan(rop: mpc_ptr);
    mpc_swap(op1: mpc_ptr, op2: mpc_ptr);

    // String Input and Output
    mpc_strtoc(rop: mpc_ptr,
               nptr: *const c_char,
               endptr: *mut *mut c_char,
               base: c_int,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_set_str(rop: mpc_ptr,
                s: *const c_char,
                base: c_int,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_get_str(b: c_int,
                n: usize,
                op: mpc_srcptr,
                rnd: mpc_rnd_t)
                -> *mut c_char;
    mpc_free_str(rop: *mut c_char);

    // Comparison Functions
    mpc_cmp(op1: mpc_srcptr, op2: mpc_srcptr) -> c_int;
    mpc_cmp_si_si(op1: mpc_srcptr, op2r: c_long, op2i: c_long) -> c_int;
}
#[inline]
pub unsafe fn mpc_cmp_si(op1: mpc_srcptr, op2: c_long) -> c_int {
    mpc_cmp_si_si(op1, op2, 0)
}
c_fn! {

    // Projection and Decomposing Functions
    mpc_real(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpc_imag(rop: mpfr_ptr, arg2: mpc_srcptr, rnd: mpfr_rnd_t) -> c_int;
}
#[inline]
pub unsafe fn mpc_realref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).re) as mpfr_ptr
}
#[inline]
pub unsafe fn mpc_imagref(op: mpc_ptr) -> mpfr_ptr {
    (&mut (*op).im) as mpfr_ptr
}
c_fn! {
    mpc_arg(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpc_proj(rop: mpc_ptr, arg2: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;

    // Basic Arithmetic Functions
    mpc_add(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_add_ui(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_ulong,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_add_fr(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpfr_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_sub(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_sub_fr(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpfr_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_fr_sub(rop: mpc_ptr,
               op1: mpfr_srcptr,
               op2: mpc_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_sub_ui(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_ulong,
               rnd: mpc_rnd_t)
               -> c_int;
}
#[inline]
pub unsafe fn mpc_mpc_ui_sub(rop: mpc_ptr,
                             op1: c_ulong,
                             op2: mpc_srcptr,
                             rnd: mpc_rnd_t)
                             -> c_int {
    mpc_ui_ui_sub(rop, op1, 0, op2, rnd)
}
c_fn! {
    mpc_ui_ui_sub(rop: mpc_ptr,
                  re1: c_ulong,
                  im1: c_ulong,
                  op2: mpc_srcptr,
                  rnd: mpc_rnd_t)
                  -> c_int;
    mpc_neg(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_mul(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_mul_ui(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_ulong,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_mul_si(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_long,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_mul_fr(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpfr_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_mul_i(rop: mpc_ptr,
              op: mpc_srcptr,
              sgn: c_int,
              rnd: mpc_rnd_t)
              -> c_int;
    mpc_sqr(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_fma(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            op3: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_div(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_div_ui(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_ulong,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_div_fr(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpfr_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_ui_div(rop: mpc_ptr,
               op1: c_ulong,
               op2: mpc_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_fr_div(rop: mpc_ptr,
               op1: mpfr_srcptr,
               op2: mpc_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_conj(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_abs(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpc_norm(rop: mpfr_ptr, op: mpc_srcptr, rnd: mpfr_rnd_t) -> c_int;
    mpc_mul_2ui(rop: mpc_ptr,
                op1: mpc_srcptr,
                op2: c_ulong,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_mul_2si(rop: mpc_ptr,
                op1: mpc_srcptr,
                op2: c_long,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_div_2ui(rop: mpc_ptr,
                op1: mpc_srcptr,
                op2: c_ulong,
                rnd: mpc_rnd_t)
                -> c_int;
    mpc_div_2si(rop: mpc_ptr,
                op1: mpc_srcptr,
                op2: c_long,
                rnd: mpc_rnd_t)
                -> c_int;

    // Power Functions and Logarithms
    mpc_sqrt(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_pow(rop: mpc_ptr,
            op1: mpc_srcptr,
            op2: mpc_srcptr,
            rnd: mpc_rnd_t)
            -> c_int;
    mpc_pow_d(rop: mpc_ptr,
              op1: mpc_srcptr,
              op2: f64,
              rnd: mpc_rnd_t)
              -> c_int;
    mpc_pow_ld(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: f64,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_pow_si(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_long,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_pow_ui(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: c_ulong,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_pow_z(rop: mpc_ptr,
              op1: mpc_srcptr,
              op2: mpz_srcptr,
              rnd: mpc_rnd_t)
              -> c_int;
    mpc_pow_fr(rop: mpc_ptr,
               op1: mpc_srcptr,
               op2: mpfr_srcptr,
               rnd: mpc_rnd_t)
               -> c_int;
    mpc_exp(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_log(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_log10(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;

    // Trigonometric Functions
    mpc_sin(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_cos(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_sin_cos(rop_sin: mpc_ptr,
                rop_cos: mpc_ptr,
                op: mpc_srcptr,
                rnd_sin: mpc_rnd_t,
                rnd_cos: mpc_rnd_t)
                -> c_int;
    mpc_tan(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_sinh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_cosh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_tanh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_asin(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_acos(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_atan(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_asinh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_acosh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;
    mpc_atanh(rop: mpc_ptr, op: mpc_srcptr, rnd: mpc_rnd_t) -> c_int;

    // Miscellaneous Functions
    mpc_urandom(rop: mpc_ptr, state: randstate_ptr) -> c_int;
    mpc_get_version() -> *const c_char;
}
pub const MPC_VERSION: c_int = (MPC_VERSION_MAJOR << 16) |
                               (MPC_VERSION_MINOR << 8) |
                               MPC_VERSION_PATCHLEVEL;
pub const MPC_VERSION_MAJOR: c_int = 1;
pub const MPC_VERSION_MINOR: c_int = 0;
pub const MPC_VERSION_PATCHLEVEL: c_int = 3;
const MPC_VERSION_BUFFER: &'static [u8] = b"1.0.3\0";
pub const MPC_VERSION_STRING: *const c_char =
    &MPC_VERSION_BUFFER[0] as *const _ as *const c_char;
#[inline]
pub fn MPC_VERSION_NUM(major: c_int, minor: c_int, patchlevel: c_int) -> c_int {
    (major << 16) | (minor << 8) | patchlevel
}
