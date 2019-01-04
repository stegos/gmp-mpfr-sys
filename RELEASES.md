<!-- Copyright © 2017–2019 University of Malta -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

Version 1.1.10 (2019-01-04)
===========================

  * Update MPFR 4.0.1-p13 -> 4.0.1-p14.
  * During Windows build use `std::os::windows::fs::symlink_dir` to
    save on some copying if allowed (Windows 1703+ developer mode).

Version 1.1.9 (2018-10-05)
==========================

  * Update MPFR 4.0.1-p11 -> 4.0.1-p13.
  * Fix function parameters that should be `intmax_t` or `uintmax_t`.

Version 1.1.8 (2018-07-23)
==========================

  * Update MPFR 4.0.1-p9 -> 4.0.1-p11.

Version 1.1.7 (2018-07-11)
==========================

  * Update MPFR 4.0.1-p6 -> 4.0.1-p9.

Version 1.1.6 (2018-05-29)
==========================

  * Automatically work around Rust issue #47048.

Version 1.1.5 (2018-05-02)
==========================

  * Update MPFR 4.0.1 -> 4.0.1-p6.

Version 1.1.4 (2018-04-23)
==========================

  * Add missing GMP, MPFR and MPC functions that take a `*mut FILE`
    argument.

Version 1.1.3 (2018-04-05)
==========================

  * Fix linkage of MPFR `uj` and `sj` functions.

Version 1.1.2 (2018-04-05)
==========================

  * Add missing MPFR and MPC functions with `uj` and `sj`, using
    `c_ulonglong` and `c_longlong` respectively.
  * Add missing MPFR `dump` function.

Version 1.1.1 (2018-02-09)
==========================

  * Update MPFR 4.0.0 -> 4.0.1.
  * Fix the type of the `tab` parameter of `mpfr::sum` to
    `*const *mut mpfr_t` instead of `*mut *mut mpfr_t`.
  * Document the `DEP_GMP_LIMB_BITS` build script metadata.
  * Add `DEP_GMP_OUT_DIR`, `DEP_GMP_LIB_DIR`, and
    `DEP_GMP_INCLUDE_DIR` build script metadata.

Version 1.1.0 (2018-01-12)
==========================

  * Update MPFR 3.1.6-p1 -> 4.0.0.
  * Update MPC 1.0.3 -> 1.1.0.
  * Deprecate and hide documentation for `mpfr::rnd_t::RNDNA`;
    `MPFR_RNDNA` is not documented by MPFR, and mpfr.h says it should
    not be used.
  * Use `c_int` instead of `#[repr(C)] enum` for the private
    enumerated type inside `#[repr(C)] struct randstate_t`.

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
