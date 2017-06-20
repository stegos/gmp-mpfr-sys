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
