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
//! not `mpfr::rnd_t::N`. Also, the types `mpfr::mpfr_t` and
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
//! To use `gmp-mpfr-sys` in your crate, add
//! `extern crate gmp_mpfr_sys;` to the crate root and add
//! `gmp-mpfr-sys` as a dependency in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gmp-mpfr-sys = "0.5.1"
//! ```
//!
//! ### Building on GNU/Linux
//!
//! To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`
//! and `make` installed on your system. For example on Fedora:
//!
//! ```sh
//! sudo dnf install diffutils gcc make
//! ```
//!
//! ### Building on macOS
//!
//! To build on macOS, you need the command-line developer tools. An easy
//! way to install them is to start building the crate using
//! `cargo build`. If the tools are not installed yet, a popup should
//! appear which should help you install them.
//!
//! ### Building on Windows
//!
//! You can build on Windows with the Rust GNU toolchain and an up-to-date
//! MSYS2 installation. Some steps for a 64-bit environment are listed
//! below, you can follow similar steps for a 32-bit environment by
//! substituting 32 for 64. To install MSYS2:
//!
//! 1. Install MSYS2 using the [installer](https://msys2.github.io/).
//!
//! 2. Launch the MSYS2 MinGW 64-bit terminal from the start menu.
//!
//! 3. Install the required tools.
//!    ```sh
//!    pacman -S pacman-mirrors
//!    pacman -S diffutils make mingw-w64-x86_64-gcc
//!    ```
//!
//! Then, to build a crate with a dependency on this crate:
//!
//! 1. Launch the MSYS MinGW 64-bit terminal from the start menu.
//!
//! 2. Change to the crate directory.
//!
//!    Note that building the GMP and MPFR libraries in MSYS with absolute
//!    paths does not work very well, so relative paths are used. If your
//!    crate is inside `C:\msys64` and the `.cargo` directory is outside
//!    `C:\msys64`, this will not work. Please move your crate to the
//!    same side of `C:\msys64` as `.cargo`.
//!
//! 3. Build the crate using `cargo`.

macro_rules! c_fn {
    {
        $($c:tt
          $name:ident($($par:ident: $ty:ty),* $(; $dots:tt)*) $(-> $ret:ty)*;
        )*
    } => {
        extern "C" {
            $(
                #[link_name = $c]
                pub fn $name($($par: $ty),* $(, $dots)*) $(-> $ret)*;
            )*
        }
    };
}

macro_rules! c_static {
    { $($c:tt $name:ident: $ty:ty;)* } => {
        extern "C" {
            $(
                #[link_name = $c]
                pub static $name: $ty;
            )*
        }
    };
}

pub mod gmp;
pub mod mpfr;
pub mod mpc;

#[cfg(test)]
mod tests {
    use gmp;
    use mpc;
    use mpfr;
    use std::f64;
    use std::mem;

    #[test]
    fn gmp_runs() {
        unsafe {
            let i = 3;
            let mut z: gmp::mpz_t = mem::uninitialized();
            let ptr = &mut z as *mut _;
            gmp::mpz_init(ptr);
            gmp::mpz_set_ui(ptr, i);
            assert!(gmp::mpz_get_ui(ptr) == i);
            gmp::mpz_clear(ptr);
        }
    }

    #[test]
    fn mpfr_runs() {
        unsafe {
            let d: f64 = 1.0 / 3.0;
            let mut fr: mpfr::mpfr_t = mem::uninitialized();
            let ptr = &mut fr as *mut _;
            mpfr::init2(ptr, 53);
            assert!(mpfr::set_d(ptr, d, mpfr::rnd_t::RNDN) == 0);
            assert!(mpfr::get_d(ptr, mpfr::rnd_t::RNDN) == d);
            mpfr::clear(ptr);
        }
    }

    #[test]
    fn mpc_runs() {
        unsafe {
            let re: f64 = 1.0 / 3.0;
            let im: f64 = f64::NEG_INFINITY;
            let mut c: mpc::mpc_t = mem::uninitialized();
            let ptr = &mut c as *mut _;
            mpc::init3(ptr, 53, 53);
            assert!(mpc::set_d_d(ptr, re, im, mpc::RNDNN) == 0);
            let re_ptr = mpc::realref(ptr);
            let im_ptr = mpc::imagref(ptr);
            assert!(mpfr::get_d(re_ptr, mpfr::rnd_t::RNDN) == re);
            assert!(mpfr::get_d(im_ptr, mpfr::rnd_t::RNDN) == im);
            mpc::clear(ptr);
        }
    }
}
