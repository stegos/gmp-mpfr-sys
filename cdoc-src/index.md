% Arbitrary-precision numbers

The following [GNU] C libraries provide numbers with arbitrarily large
precision:

* [GMP] for integers and rational numbers,
* [MPFR] for floating-point numbers, and
* [MPC] for complex numbers.

Low-level Rust bindings are provided in the [gmp-mpfr-sys][sys crate]
crate. This crate can be used to write higher-level bindings, or to
use the C functions directly.

The [Rug][rug crate] crate is a high-level Rust library that provides:

* bignum [integers][`Integer`] with arbitrary precision,
* bignum [rational numbers][`Rational`] with arbitrary precision,
* multi-precision [floating-point numbers][`Float`] with correct
  rounding, and
* multi-precision [complex numbers][`Complex`] with correct rounding.

## Documentation

C libraries:

* [GMP][gmp doc]
* [MPFR][mpfr doc]
* [MPC][mpc doc]

Rust crates:

* [gmp-mpfr-sys][sys]
* [Rug][rug]

## License

These libraries are free software: you can redistribute them and/or
modify them under the terms of the GNU Lesser General Public License
as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version. See the full text of
the [GNU LGPL] and [GNU GPL] for details.

[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: http://www.multiprecision.org/mpc/
[MPFR]: http://www.mpfr.org/
[`Complex`]: https://docs.rs/rug/*/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/*/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/*/rug/struct.Integer.html
[`Rational`]: https://docs.rs/rug/*/rug/struct.Rational.html
[gmp doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/gmp/index.html
[mpc doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpc/index.html
[mpfr doc]: https://tspiteri.gitlab.io/gmp-mpfr-sys/mpfr/index.html
[rug crate]: https://crates.io/crates/rug
[rug]: https://docs.rs/rug/*/rug/index.html
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
[sys]: https://docs.rs/gmp-mpfr-sys/~1.1/gmp_mpfr_sys/index.html
