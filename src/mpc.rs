/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use ::gmp::*;
use ::mpfr::*;

pub type mpc_rnd_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __mpc_struct {
    pub re: mpfr_t,
    pub im: mpfr_t,
}
impl ::std::default::Default for __mpc_struct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type mpc_t = [__mpc_struct; 1usize];
pub type mpc_ptr = *mut __mpc_struct;
pub type mpc_srcptr = *const __mpc_struct;
#[link(name = "mpc", kind = "static")]
extern "C" {
    pub fn mpc_add(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_add_fr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpfr_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_add_si(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_add_ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sub(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_sub_fr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpfr_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_fr_sub(arg1: mpc_ptr, arg2: mpfr_srcptr, arg3: mpc_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_sub_ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_ui_ui_sub(arg1: mpc_ptr, arg2: ::std::os::raw::c_ulong,
                         arg3: ::std::os::raw::c_ulong, arg4: mpc_srcptr,
                         arg5: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_mul(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_mul_fr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpfr_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_mul_ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_mul_si(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_mul_i(arg1: mpc_ptr, arg2: mpc_srcptr,
                     arg3: ::std::os::raw::c_int, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sqr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_div(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_pow(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_pow_fr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpfr_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_pow_ld(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: f64,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_pow_d(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: f64,
                     arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_pow_si(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_pow_ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_pow_z(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpz_srcptr,
                     arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_div_fr(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpfr_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_fr_div(arg1: mpc_ptr, arg2: mpfr_srcptr, arg3: mpc_srcptr,
                      arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_div_ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                      arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_ui_div(arg1: mpc_ptr, arg2: ::std::os::raw::c_ulong,
                      arg3: mpc_srcptr, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_div_2ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                       arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_mul_2ui(arg1: mpc_ptr, arg2: mpc_srcptr,
                       arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_div_2si(arg1: mpc_ptr, arg2: mpc_srcptr,
                       arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_mul_2si(arg1: mpc_ptr, arg2: mpc_srcptr,
                       arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_conj(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_neg(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_norm(arg1: mpfr_ptr, arg2: mpc_srcptr, arg3: mpfr_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_abs(arg1: mpfr_ptr, arg2: mpc_srcptr, arg3: mpfr_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sqrt(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_d(arg1: mpc_ptr, arg2: f64, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_d_d(arg1: mpc_ptr, arg2: f64, arg3: f64, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_ld(arg1: mpc_ptr, arg2: f64, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_ld_ld(arg1: mpc_ptr, arg2: f64, arg3: f64, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_f(arg1: mpc_ptr, arg2: mpf_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_f_f(arg1: mpc_ptr, arg2: mpf_srcptr, arg3: mpf_srcptr,
                       arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_set_fr(arg1: mpc_ptr, arg2: mpfr_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_fr_fr(arg1: mpc_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                         arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_set_q(arg1: mpc_ptr, arg2: mpq_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_q_q(arg1: mpc_ptr, arg2: mpq_srcptr, arg3: mpq_srcptr,
                       arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_set_si(arg1: mpc_ptr, arg2: ::std::os::raw::c_long,
                      arg3: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_set_si_si(arg1: mpc_ptr, arg2: ::std::os::raw::c_long,
                         arg3: ::std::os::raw::c_long, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_ui(arg1: mpc_ptr, arg2: ::std::os::raw::c_ulong,
                      arg3: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_set_ui_ui(arg1: mpc_ptr, arg2: ::std::os::raw::c_ulong,
                         arg3: ::std::os::raw::c_ulong, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_z(arg1: mpc_ptr, arg2: mpz_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_z_z(arg1: mpc_ptr, arg2: mpz_srcptr, arg3: mpz_srcptr,
                       arg4: mpc_rnd_t) -> ::std::os::raw::c_int;
    pub fn mpc_swap(arg1: mpc_ptr, arg2: mpc_ptr);
    pub fn mpc_fma(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_srcptr,
                   arg4: mpc_srcptr, arg5: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_nan(arg1: mpc_ptr);
    pub fn mpc_real(arg1: mpfr_ptr, arg2: mpc_srcptr, arg3: mpfr_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_imag(arg1: mpfr_ptr, arg2: mpc_srcptr, arg3: mpfr_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_arg(arg1: mpfr_ptr, arg2: mpc_srcptr, arg3: mpfr_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_proj(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_cmp(arg1: mpc_srcptr, arg2: mpc_srcptr)
     -> ::std::os::raw::c_int;
    pub fn mpc_cmp_si_si(arg1: mpc_srcptr, arg2: ::std::os::raw::c_long,
                         arg3: ::std::os::raw::c_long)
     -> ::std::os::raw::c_int;
    pub fn mpc_exp(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_log(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_log10(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sin(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_cos(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sin_cos(arg1: mpc_ptr, arg2: mpc_ptr, arg3: mpc_srcptr,
                       arg4: mpc_rnd_t, arg5: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_tan(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_sinh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_cosh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_tanh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_asin(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_acos(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_atan(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_asinh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_acosh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_atanh(arg1: mpc_ptr, arg2: mpc_srcptr, arg3: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_clear(arg1: mpc_ptr);
    pub fn mpc_urandom(arg1: mpc_ptr, arg2: gmp_randstate_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_init2(arg1: mpc_ptr, arg2: mpfr_prec_t);
    pub fn mpc_init3(arg1: mpc_ptr, arg2: mpfr_prec_t, arg3: mpfr_prec_t);
    pub fn mpc_get_prec(x: mpc_srcptr) -> mpfr_prec_t;
    pub fn mpc_get_prec2(pr: *mut mpfr_prec_t, pi: *mut mpfr_prec_t,
                         x: mpc_srcptr);
    pub fn mpc_set_prec(arg1: mpc_ptr, arg2: mpfr_prec_t);
    pub fn mpc_get_version() -> *const ::std::os::raw::c_char;
    pub fn mpc_strtoc(arg1: mpc_ptr, arg2: *const ::std::os::raw::c_char,
                      arg3: *mut *mut ::std::os::raw::c_char,
                      arg4: ::std::os::raw::c_int, arg5: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_set_str(arg1: mpc_ptr, arg2: *const ::std::os::raw::c_char,
                       arg3: ::std::os::raw::c_int, arg4: mpc_rnd_t)
     -> ::std::os::raw::c_int;
    pub fn mpc_get_str(arg1: ::std::os::raw::c_int, arg2: size_t,
                       arg3: mpc_srcptr, arg4: mpc_rnd_t)
     -> *mut ::std::os::raw::c_char;
    pub fn mpc_free_str(arg1: *mut ::std::os::raw::c_char);
}
