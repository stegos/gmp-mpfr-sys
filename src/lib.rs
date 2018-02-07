// Copyright © 2017–2018 University of Malta

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
//! The `gmp-mpfr-sys` crate provides Rust FFI bindings for:
//!
//! * the [GNU Multiple Precision Arithmetic Library][gmp] (GMP)
//!   version 6.1.2, a library for arbitrary precision arithmetic,
//! * the [GNU MPFR Library][mpfr] version 4.0.1, a library for
//!   multiple-precision floating-point computations with correct
//!   rounding, and
//! * [GNU MPC][mpc] version 1.1.0, a library for the arithmetic of
//!   complex numbers with arbitrarily high precision and correct
//!   rounding.
//!
//! The source of the three libraries is included in the package.
//!
//! This crate is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version. See the full text
//! of the [GNU LGPL][lgpl] and [GNU GPL][gpl] for details.
//!
//! ## Basic features
//!
//! This crate provides a low-level interface to GMP, MPFR and MPC in
//! three modules. The documentation of the three modules contains
//! links for each function, constant and type into the respective
//! documentation of [GMP][gmp doc], [MPFR][mpfr doc] and
//! [MPC][mpc doc] libraries. The three modules of this crate are:
//!
//! * [`gmp`][sys gmp] provides external FFI bindings to GMP.
//! * [`mpfr`][sys mpfr] provides external FFI bindings to MPFR.
//! * [`mpc`][sys mpc] provides external FFI bindings to MPC.
//!
//! If you want a high-level API, consider using the [`rug`][rug] crate,
//! which provides big integer and floating-point numbers. Its main
//! features are:
//!
//! * big [integers][rug int] with arbitrary precision based on GMP,
//! * big [rational numbers][rug rat] with arbitrary precision based
//!   on GMP,
//! * multi-precision [floating-point numbers][rug flo] with correct
//!   rounding based on MPFR, and
//! * multi-precision [complex numbers][rug com] with correct rounding
//!   based on MPC.
//!
//! ## Notes
//!
//! ### Name prefixes
//!
//! Since modules and enumerated types provide namespacing, most
//! prefixes in the C names are removed. However, when the prefix is
//! not a whole word it is not removed, for example
//! `mp_set_memory_functions()` becomes `gmp::set_memory_functions()`,
//! but `mpz_init()` becomes `gmp::mpz_init()` not `gmp::z_init()`,
//! and `MPFR_RNDN` in `enum MPFR_RND_T` becomes `mpfr::rnd_t::RNDN`
//! not `mpfr::rnd_t::N`.  Also, the types `mpfr::mpfr_t` and
//! `mpc::mpc_t` are *not* shortened to `mpfr::t` or `mpc::t`.
//!
//! ### Types
//!
//! Unlike in the C libraries, the types `gmp::mpz_t`, `gmp::mpq_t`,
//! `gmp::mpf_t`, `gmp::rand_state_t`, `mpfr::mpfr_t` and `mpc::mpc_t`
//! are defined directly as structs, not as single-element arrays.
//!
//! ### Undocumented or obsolete functions
//!
//! The bindings do not cover undocumented or obsolete functions and
//! macros.
//!
//! ## Usage
//!
//! This crate required rustc version 1.13.0 or later.
//!
//! To use `gmp-mpfr-sys` in your crate, add
//! `extern crate gmp_mpfr_sys;` to the crate root and add
//! `gmp-mpfr-sys` as a dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gmp-mpfr-sys = "1.1.0"
//! ```
//!
//! If the C libraries have a major version bump with some deprecated
//! functions removed, but no features are removed in the Rust
//! bindings, then `gmp-mpfr-sys` will have a minor version bump
//! rather than a major version bump. This allows more compatiblity
//! across crates that use the Rust bindings but do not use the C
//! libraries directly.
//!
//! If on the other hand a dependent crate includes a C library that
//! directly uses the header (*.h*) and library (*.a*) files built
//! using C, it can be a good idea to depend on version `"~1.1.0"`
//! instead of version `"1.1.0"` in order to ensure backwards
//! compatibility at the C level as well.
//!
//! ### Metadata
//!
//! The `gmp-mpfr-sys` crate passes some metadata to its dependents:
//!
//! 1. `DEP_GMP_LIMB_BITS` contains the number of bits per limb, which
//!    is 32 or 64.
//! 2. `DEP_GMP_OUT_DIR` contains the path of a directory that
//!    contains two subdirectories: the first subdirectory is named
//!    *lib* and contains the generated library (*.a*) files, and the
//!    second subdirectory is named *include* and contains the
//!    corresponding header (*.h*) files.
//! 3. `DEP_GMP_LIB_DIR` contains the path of the *lib* subdirectory
//!    of the `DEP_GMP_OUT_DIR` directory.
//! 4. `DEP_GMP_INCLUDE_DIR` contains the path of the *include*
//!    subdirectory of the `DEP_GMP_OUT_DIR` directory.
//!
//! A dependent crate can use these environment variables in its build
//! script.
//!
//! ### Optional features
//!
//! The `gmp-mpfr-sys` crate has two optional features `mpfr` and
//! `mpc` to include the MPFR and MPC libraries respectively. The GMP
//! library is always included. The optional features are enabled by
//! default; to disable them add this to `Cargo.toml`:
//!
//! ```toml
//! [dependencies.gmp-mpfr-sys]
//! version = "1.1.0"
//! default-features = false
//! ```
//!
//! To use features selectively, you can add this to `Cargo.toml`:
//!
//! ```toml
//! [dependencies.gmp-mpfr-sys]
//! version = "1.1.0"
//! default-features = false
//! # Pick which features to use
//! features = ["mpfr"]
//! ```
//!
//! Note that the the `mpc` feature depends on, and will enable, the
//! `mpfr` feature.
//!
//! ### Building on GNU/Linux
//!
//! To build on GNU/Linux, simply make sure you have `diffutils`,
//! `gcc`, `make` and `m4` installed on your system. For example on
//! Fedora:
//!
//! ```sh
//! sudo dnf install diffutils gcc make m4
//! ```
//!
//! ### Building on macOS
//!
//! To build on macOS, you need the command-line developer tools. An
//! easy way to install them is to start building the crate using
//! `cargo build`. If the tools are not installed yet, a popup should
//! appear which should help you install them.
//!
//! ### Building on Windows
//!
//! You can build on Windows with the Rust GNU toolchain and an
//! up-to-date MSYS2 installation. Some steps for a 64-bit environment
//! are listed below. (32-bit: Changes for a 32-bit environment are
//! written in brackets like this comment.)
//!
//! To install MSYS2:
//!
//! 1. Install MSYS2 using the [installer][msys].
//!
//! 2. Launch the MSYS2 MinGW 64-bit terminal from the start
//!    menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)
//!
//! 3. Install the required tools.
//!
//!    ```sh
//!    pacman -S pacman-mirrors
//!    pacman -S diffutils make mingw-w64-x86_64-gcc
//!    ```
//!
//!    (32-bit: Install `mingw-w64-i686-gcc` instead of
//!    `mingw-w64-x86_64-gcc`.)
//!
//! Then, to build a crate with a dependency on this crate:
//!
//! 1. Launch the MSYS MinGW 64-bit terminal from the start menu.
//!    (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)
//!
//! 2. Change to the crate directory.
//!
//! 3. Build the crate using `cargo`.
//!
//! [gmp doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/index.html
//! [gmp]:      https://gmplib.org/
//! [gpl]:      https://www.gnu.org/licenses/gpl-3.0.html
//! [lgpl]:     https://www.gnu.org/licenses/lgpl-3.0.en.html
//! [mpc doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/mpc/index.html
//! [mpc]:      http://www.multiprecision.org/mpc/
//! [mpfr doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/index.html
//! [mpfr]:     http://www.mpfr.org/
//! [msys]:     https://msys2.github.io/
//! [rug com]:  https://docs.rs/rug/*/rug/struct.Complex.html
//! [rug flo]:  https://docs.rs/rug/*/rug/struct.Float.html
//! [rug int]:  https://docs.rs/rug/*/rug/struct.Integer.html
//! [rug rat]:  https://docs.rs/rug/*/rug/struct.Rational.html
//! [rug]:      https://docs.rs/rug/*/rug/index.html
//! [sys gmp]:  gmp/index.html
//! [sys mpc]:  mpc/index.html
//! [sys mpfr]: mpfr/index.html
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/gmp-mpfr-sys/~1.1.0")]
#![doc(test(attr(deny(warnings))))]

pub mod gmp;
#[cfg(feature = "mpfr")]
pub mod mpfr;
#[cfg(feature = "mpc")]
pub mod mpc;
