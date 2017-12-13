# Rust low-level bindings for GMP, MPFR and MPC

The `gmp-mpfr-sys` crate provides Rust FFI bindings for:

* the [GNU Multiple Precision Arithmetic Library][gmp] (GMP),
* the [GNU MPFR Library][mpfr], a library for multiple-precision
  floating-point computations, and
* [GNU MPC][mpc], a library for the arithmetic of complex numbers with
  arbitrarily high precision.

The source of the three libraries is included in the package.

This crate is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version. See the full text of the
[GNU LGPL][lgpl] and [GNU GPL][gpl] for details.

## Basic use

This crate provides a low-level interface to GMP, MPFR and MPC in
three modules. The [documentation][sys] of the three modules contains
links for each function, constant and type into the respective
documentation of [GMP][gmp doc], [MPFR][mpfr doc] and [MPC][mpc doc]
libraries. The three modules of this crate are:

* [`gmp`][sys gmp] provides external FFI bindings to GMP.
* [`mpfr`][sys mpfr] provides external FFI bindings to MPFR.
* [`mpc`][sys mpc] provides external FFI bindings to MPC.

If you want a high-level API, consider using the [`rug`][rug] crate,
which provides big integer and floating-point numbers. Its main
features are:

* big [integers][rug int] with arbitrary precision based on GMP,
* big [rational numbers][rug rat] with arbitrary precision based on
  GMP,
* multi-precision [floating-point numbers][rug flo] with correct
  rounding based on MPFR, and
* multi-precision [complex numbers][rug com] with correct rounding
  based on MPC.

## Notes

### Name prefixes

Since modules and enumerated types provide namespacing, most prefixes
in the C names are removed. However, when the prefix is not a whole
word it is not removed, for example `mp_set_memory_functions()`
becomes `gmp::set_memory_functions()`, but `mpz_init()` becomes
`gmp::mpz_init()` not `gmp::z_init()`, and `MPFR_RNDN` in
`enum MPFR_RND_T` becomes `mpfr::rnd_t::RNDN` not `mpfr::rnd_t::N`.
Also, the types `mpfr::mpfr_t` and `mpc::mpc_t` are *not* shortened to
`mpfr::t` or `mpc::t`.

### Types

Unlike in the C libraries, the types `gmp::mpz_t`, `gmp::mpq_t`,
`gmp::mpf_t`, `gmp::rand_state_t`, `mpfr::mpfr_t` and `mpc::mpc_t` are
defined directly as structs, not as single-element arrays.

### Undocumented or obsolete functions

The bindings do not cover undocumented or obsolete functions and
macros.

## Usage

To use `gmp-mpfr-sys` in your crate, add `extern crate gmp_mpfr_sys;`
to the crate root and add `gmp-mpfr-sys` as a dependency in
`Cargo.toml`:

```toml
[dependencies]
gmp-mpfr-sys = "1.0"
```

### Optional features

The `gmp-mpfr-sys` crate has two optional features `mpfr` and `mpc` to
include the MPFR and MPC libraries respectively. The GMP library is
always included. The optional features are enabled by default; to
disable them add this to `Cargo.toml`:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.0"
default-features = false
```

To use features selectively, you can add this to `Cargo.toml`:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.0"
default-features = false
# Pick which features to use
features = ["mpfr"]
```

Note that the the `mpc` feature depends on, and will enable, the
`mpfr` feature.

### Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`
and `make` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc make
```

### Building on macOS

To build on macOS, you need the command-line developer tools. An easy
way to install them is to start building the crate using
`cargo build`. If the tools are not installed yet, a popup should
appear which should help you install them.

### Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below. (32-bit: Changes for a 32-bit environment are written in
brackets like this comment.)

To install MSYS2:

1. Install MSYS2 using the [installer][msys].

2. Launch the MSYS2 MinGW 64-bit terminal from the start
   menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

3. Install the required tools.

   ```sh
   pacman -S pacman-mirrors
   pacman -S diffutils make mingw-w64-x86_64-gcc
   ```
   
   (32-bit: Install `mingw-w64-i686-gcc` instead of
   `mingw-w64-x86_64-gcc`.)
   
Then, to build a crate with a dependency on this crate:

1. Launch the MSYS MinGW 64-bit terminal from the start menu. (32-bit:
   Launch the MSYS2 MinGW 32-bit terminal instead.)

2. Change to the crate directory.

   Note that building the GMP, MPFR and MPC libraries in MSYS with
   absolute paths does not work very well, so relative paths are
   used. If your crate is inside `C:\msys64` and the `.cargo`
   directory is outside `C:\msys64`, this will not work. Please move
   your crate to the same side of `C:\msys64` as `.cargo`.

3. Build the crate using `cargo`.

[gmp doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/index.html
[gmp]:      https://gmplib.org/
[gpl]:      https://www.gnu.org/licenses/gpl-3.0.html
[lgpl]:     https://www.gnu.org/licenses/lgpl-3.0.en.html
[mpc doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/mpc/index.html
[mpc]:      http://www.multiprecision.org/
[mpfr doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/index.html
[mpfr]:     http://www.mpfr.org/
[msys]:     https://msys2.github.io/
[rug com]:  https://docs.rs/rug/*/rug/struct.Complex.html
[rug flo]:  https://docs.rs/rug/*/rug/struct.Float.html
[rug int]:  https://docs.rs/rug/*/rug/struct.Integer.html
[rug rat]:  https://docs.rs/rug/*/rug/struct.Rational.html
[rug]:      https://docs.rs/rug/*/rug/index.html
[sys gmp]:  https://docs.rs/gmp-mpfr-sys/~1.0.8/gmp_mpfr_sys/gmp/index.html
[sys mpc]:  https://docs.rs/gmp-mpfr-sys/~1.0.8/gmp_mpfr_sys/mpc/index.html
[sys mpfr]: https://docs.rs/gmp-mpfr-sys/~1.0.8/gmp_mpfr_sys/mpfr/index.html
[sys]:      https://docs.rs/gmp-mpfr-sys/~1.0.8/gmp_mpfr_sys/index.html
