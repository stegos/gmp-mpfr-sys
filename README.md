# Rust low-level bindings for GMP, MPFR and MPC

The gmp-mpfr-sys crate provides Rust FFI bindings to the following
[GNU] arbitrary-precision libraries:

* [GMP] for integers and rational numbers,
* [MPFR] for floating-point numbers, and
* [MPC] for complex numbers.

The source of the three libraries is included in the package.

The gmp-mpfr-sys crate is free software: you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version. See the full
text of the [GNU LGPL] and [GNU GPL] for details.

## What’s new

### Version 1.1.3 news

* Some missing functions were added to the [`mpfr`] module: [`dump`],
  [`get_sj`], [`get_uj`], [`set_sj_2exp`], [`set_sj`][`mpfr::set_sj`],
  [`set_uj_2exp`] and [`set_uj`][`mpfr::set_uj`].
* Some missing functions were added to the [`mpc`] module:
  [`set_sj_sj`], [`set_sj`][`mpc::set_sj`], [`set_uj_uj`] and
  [`set_uj`][`mpc::set_uj`].

### Older releases

Details on older releases can be found in [*RELEASES.md*].

[*RELEASES.md*]: https://gitlab.com/tspiteri/gmp-mpfr-sys/blob/master/RELEASES.md
[`dump`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.dump.html
[`get_sj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.get_sj.html
[`get_uj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.get_uj.html
[`set_sj_2exp`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.set_sj_2exp.html
[`mpfr::set_sj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.set_sj.html
[`set_uj_2exp`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.set_uj_2exp.html
[`mpfr::set_uj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/fn.set_uj.html
[`set_sj_sj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/fn.set_sj_sj.html
[`mpc::set_sj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/fn.set_sj.html
[`set_uj_uj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/fn.set_uj_uj.html
[`mpc::set_uj`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/fn.set_uj.html

## Basic features

This crate contains three modules:

* [`gmp`] provides external FFI bindings to [GMP].
* [`mpfr`] provides external FFI bindings to [MPFR].
* [`mpc`] provides external FFI bindings to [MPC].

If you want a high-level API, consider using [Rug][rug crate], a crate
which provides big integer and floating-point numbers. Its main
features are

* bignum [integers][`Integer`] with arbitrary precision,
* bignum [rational numbers][`Rational`] with arbitrary precision,
* multi-precision [floating-point numbers][`Float`] with correct
  rounding, and
* multi-precision [complex numbers][`Complex`] with correct rounding.

### Name prefixes

Since modules and enumerated types provide namespacing, most prefixes
in the C names are removed. However, when the prefix is not a whole
word it is not removed. For example [`mp_set_memory_functions`]
becomes [`gmp::set_memory_functions`], but [`mpz_init`] becomes
[`gmp::mpz_init`] not `gmp::z_init`, and [`MPFR_RNDN`] in
[`enum MPFR_RND_T`] becomes [`mpfr::rnd_t::RNDN`] not
`mpfr::rnd_t::N`. Also, the types [`mpfr::mpfr_t`] and [`mpc::mpc_t`]
are *not* shortened to `mpfr::t` or `mpc::t`.

### Types

Unlike in the C libraries, the types [`gmp::mpz_t`], [`gmp::mpq_t`],
[`gmp::mpf_t`], [`gmp::randstate_t`], [`mpfr::mpfr_t`] and
[`mpc::mpc_t`] are defined directly as structs, not as single-element
arrays.

### Undocumented or obsolete functions

The bindings do not cover undocumented or obsolete functions and
macros.

## Using gmp-mpfr-sys

The gmp-mpfr-sys crate is available on [crates.io][sys crate]. To use
gmp-mpfr-sys in your crate, add it as a dependency inside
[*Cargo.toml*]:

```toml
[dependencies]
gmp-mpfr-sys = "1.1"
```

You also need to declare it by adding this to your crate root (usually
*lib.rs* or *main.rs*):

```rust
extern crate gmp_mpfr_sys;
```

This crate required rustc version 1.13.0 or later.

If the C libraries have a major version bump with some deprecated
functions removed, but no features are removed in the Rust bindings,
then gmp-mpfr-sys will have a minor version bump rather than a major
version bump. This allows more compatiblity across crates that use the
Rust bindings but do not use the C libraries directly.

If on the other hand a dependent crate includes a C library that
directly uses the header (*.h*) and library (*.a*) files built using
C, it can be a good idea to depend on version `"~1.1"` instead of
version `"1.1"` in order to ensure backwards compatibility at the C
level as well.

## Optional features

The gmp-mpfr-sys crate has two optional features:

1. `mpfr`, enabled by default. Required to include the [MPFR] library.
2. `mpc`, enabled by default. Required to include the [MPC] library.
   This feature requires the `mpfr` feature.

The [GMP] library is always included.

The two optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.1"
default-features = false
features = ["mpfr"]
```

Here only the `mpfr` feature is selected.

## Metadata

The gmp-mpfr-sys crate passes some metadata to its dependents:

1. `DEP_GMP_LIMB_BITS` contains the number of bits per limb, which is
   32 or 64.
2. `DEP_GMP_OUT_DIR` contains the path of a directory that contains
   two subdirectories: the first subdirectory is named *lib* and
   contains the generated library (*.a*) files, and the second
   subdirectory is named *include* and contains the corresponding
   header (*.h*) files.
3. `DEP_GMP_LIB_DIR` contains the path of the *lib* subdirectory of
   the `DEP_GMP_OUT_DIR` directory.
4. `DEP_GMP_INCLUDE_DIR` contains the path of the *include*
   subdirectory of the `DEP_GMP_OUT_DIR` directory.

A dependent crate can use these environment variables in its build
script.

## Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
`make` and `m4` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc make m4
```

## Building on macOS

To build on macOS, you need the command-line developer tools. An easy
way to install them is to start building the crate using
`cargo build`. If the tools are not installed yet, a popup should
appear which should help you install them.

## Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below. (32-bit: Changes for a 32-bit environment are written in
brackets like this comment.)

To install MSYS2:

1.  Install MSYS2 using the [installer][msys].

2.  Launch the MSYS2 MinGW 64-bit terminal from the start
    menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

3.  Install the required tools.

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

3. Build the crate using `cargo`.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: http://www.multiprecision.org/mpc/
[MPFR]: http://www.mpfr.org/
[`Complex`]: https://docs.rs/rug/*/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/*/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/*/rug/struct.Integer.html
[`MPFR_RNDN`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/MPFR-Basics.html#Rounding-Modes
[`Rational`]: https://docs.rs/rug/*/rug/struct.Rational.html
[`enum MPFR_RND_T`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/MPFR-Basics.html#index-mpfr_005frnd_005ft
[`gmp::mpf_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.mpf_t.html
[`gmp::mpq_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.mpq_t.html
[`gmp::mpz_init`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/fn.mpz_init.html
[`gmp::mpz_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.mpz_t.html
[`gmp::randstate_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/struct.randstate_t.html
[`gmp::set_memory_functions`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/fn.set_memory_functions.html
[`gmp`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/gmp/index.html
[`mp_set_memory_functions`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/Custom-Allocation.html#index-mp_005fset_005fmemory_005ffunctions
[`mpc::mpc_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/struct.mpc_t.html
[`mpc`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpc/index.html
[`mpfr::mpfr_t`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/struct.mpfr_t.html
[`mpfr::rnd_t::RNDN`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/enum.rnd_t.html#variant.RNDN
[`mpfr`]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/mpfr/index.html
[`mpz_init`]: https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/Integer-Functions.html#index-mpz_005finit
[msys]:     https://msys2.github.io/
[rug crate]: https://crates.io/crates/rug
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
