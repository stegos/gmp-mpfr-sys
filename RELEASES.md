Version 1.1.0
=============

* Requires rustc version 1.18.0.
* Update MPFR 3.1.6-p1 -> 4.0.0.
* Update MPC 1.0.3 -> 1.1.
* Deprecate and hide documentation for `mpfr::rnd_t::RNDNA`;
  `MPFR_RNDNA` is not documented by MPFR, and mpfr.h says it should
  not be used.
* Use `c_int` instead of `#[repr(C)] enum` for the private enumerated
  type inside `#[repr(C)] struct randstate_t`.

Version 1.0.8 (2017-11-08)
==========================

* Update MPFR 3.1.6 -> 3.1.6-p1.

Version 1.0.7 (2017-09-10)
==========================

* Update MPFR 3.1.5-p9 -> 3.1.6.

Version 1.0.6 (2017-07-24)
==========================

* Update MPFR 3.1.5-p8 -> 3.1.5-p9.

Version 1.0.5 (2017-06-26)
==========================

* Bug fix: use C linkage for inline functions.

Version 1.0.4 (2017-06-20)
==========================

* Update MPFR 3.1.5 -> 3.1.5-p8.

Version 1.0.3 (2017-06-06)
==========================

* Add `gmp::mpq_numref_const()`, `gmp::mpq_denref_const()`,
  `mpc::realref_const()` and `mpc::imagref_const()`.
* Add inline version of functions which are inline in gmp.h, mpfr.h.
* Bug fix: `gmp::mpz_even_p()`.

Version 1.0.2 (2017-05-20)
==========================

* Add features `mpfr` and `mpc`, which are enabled by default, to
  allow opting out of the MPFR and MPC libraries.

Version 1.0.1 (2017-05-06)
==========================

* Expliciltly link to gcc_eh and pthread under MinGW.

Version 1.0.0 (2017-04-24)
==========================

* GMP 6.1.2, MPFR 3.1.5, MPC 1.0.3
