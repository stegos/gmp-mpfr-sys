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

//! # Rust low-level bindings for GMP, MPFR and MPC
//!
//! The `gmp-mpfr-sys` crate provides Rust low-level bindings for
//!
//! * the [GNU Multiple Precision Arithmetic Library]
//!   (https://gmplib.org/) (GMP),
//! * the [GNU MPFR Library](http://www.mpfr.org/), a library for
//!   multiple-precision floating-point computations, and
//! * [GNU MPC](http://www.multiprecision.org/), a library for the
//!   arithmetic of complex numbers with arbitrarily high precision.
//!
//! This crate is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.
//!
//! This crate provides a low-level interface to GMP, MPFR and MPC. If you
//! want a high-level API, consider using the following crates:
//!
//! * [`rugint`](https://tspiteri.gitlab.io/gmp-mpfr/rugint/)
//!   provides arbitrary-precision integers based on GMP.
//! * [`rugrat`](https://tspiteri.gitlab.io/gmp-mpfr/rugrat/)
//!   provides arbitrary-precision rational number based on GMP.
//! * [`rugflo`](https://tspiteri.gitlab.io/gmp-mpfr/rugflo/)
//!   provides arbitrary-precision floating-point numbers based on MPFR.
//! * [`rugcom`](https://tspiteri.gitlab.io/gmp-mpfr/rugcom/)
//!   provides arbitrary-precision complex numbers based on MPC.
//!
//! If you want to use the low-level bindings, you can refer to the
//! documentation of the C libraries themselves:
//!
//! * [GMP](https://gmplib.org/manual/)
//! * [MPFR](http://www.mpfr.org/mpfr-current/mpfr.html)
//! * [MPC](http://www.multiprecision.org/index.php?prog=mpc&page=html)
//!
//! ## Name prefixes
//!
//! Since modules and enumerated types provide namespacing, most
//! prefixes in the C names are removed. However, when the prefix is
//! not a whole word it is not removed, for example
//! `mp_set_memory_functions()` becomes `gmp::set_memory_functions()`,
//! but `mpz_init()` becomes `gmp::mpz_init()` not `gmp::z_init()`,
//! and `MPFR_RNDN` in `enum MPFR_RND_T` becomes `mpfr::rnd_t::RNDN`
//! not `mpfr::rnd_t::N`. Also, the types `mpfr::mpfr_t` and
//! `mpc::mpc_t` are *not* shortened to `mpfr::t` or `mpc::t`.
//!
//! ## Types
//!
//! Unlike the C libraries, the types `gmp::mpz_t`, `gmp::mpq_t`,
//! `gmp::mpf_t`, `gmp::rand_state_t`, `mpfr::mpfr_t` and `mpc::mpc_t`
//! are defined directly as structs, not as single-element arrays.
//!
//! ## Undocumented or obsolete functions
//!
//! The bindings do not cover undocumented or obsolete functions and
//! macros.

pub mod gmp;
pub mod mpfr;
pub mod mpc;
