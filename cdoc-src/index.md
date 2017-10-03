% Arbitrary-precision numbers

The following C libraries provide numbers with arbitrarily large
precision:

* the [GNU Multiple Precision Arithmetic Library][gmp] (GMP),
* the [GNU MPFR Library][mpfr], a library for multiple-precision
  floating-point computations, and
* [GNU MPC][mpc], a library for the arithmetic of complex numbers with
  arbitrarily high precision.

Low-level Rust bindings are provided in the [`gmp-mpfr-sys`][sys]
crate. This crate can be used to write higher-level bindings, or to
use the C functions directly.

The [`rug`][rug] crate is a high-level Rust library that provides:

* big [integers][rug int] with arbitrarily large precision,
* big [rational numbers][rug rat] with arbitrarily large precision,
* multi-precision [floating-point numbers][rug flo] with correct
  rounding, and
* multi-precision [complex numbers][rug com] with correct rounding.

## Documentation

C libraries:

* [GMP][gmp doc]
* [MPFR][mpfr doc]
* [MPC][mpc doc]

Rust crates:

* [`gmp-mpfr-sys`][sys]
* [`rug`][rug]

## License

These libraries are free software: you can redistribute them and/or
modify them under the terms of the GNU Lesser General Public License
as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version. See the full text of
the [GNU LGPL][lgpl] and [GNU GPL][gpl] for details.

[gmp doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/index.html
[gmp]:      https://gmplib.org/
[gpl]:      https://www.gnu.org/licenses/gpl-3.0.html
[lgpl]:     https://www.gnu.org/licenses/lgpl-3.0.en.html
[mpc doc]:  https://tspiteri.gitlab.io/gmp-mpfr-sys/mpc/index.html
[mpc]:      http://www.multiprecision.org/
[mpfr doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/index.html
[mpfr]:     http://www.mpfr.org/
[rug com]:  https://docs.rs/rug/*/rug/struct.Complex.html
[rug flo]:  https://docs.rs/rug/*/rug/struct.Float.html
[rug int]:  https://docs.rs/rug/*/rug/struct.Integer.html
[rug rat]:  https://docs.rs/rug/*/rug/struct.Rational.html
[rug]:      https://docs.rs/rug/*/
[sys]:      https://docs.rs/gmp-mpfr-sys/^1.0.7/
