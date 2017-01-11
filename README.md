# Rust low-level bindings for GMP and MPFR

The `gmp-mpfr-sys` crate provides Rust low-level bindings for the
[GNU Multiple Precision Arithmetic Library](https://gmplib.org/) (GMP)
and the [GNU MPFR Library](http://www.mpfr.org/), a library for
multiple-precision floating-point computations. The source of the two
libraries is included in the package.

## License

Just like GMP and MPFR, this crate is free software: you can
redistribute it and/or modify it under the terms of either

* the GNU Lesser General Public License as published by the Free
  Software Foundation, either version 3 of the License, or (at your
  option) any later version, or
* the GNU General Public License as published by the Free Software
  Foundation, either version 3 of the License, or (at your option) any
  later version.
  
See [LICENSE-LGPL](LICENSE-LGPL.md) and [LICENSE-GPL](LICENSE-GPL.md)
for details.

## Building on Windows

You can build on Windows with the rust gnu toolchain and an up-to-date
MSYS2 installation. To install MSYS2, you can do the following:

1. Install MSYS2 using the [installer](https://msys2.github.io/).

2. Launch the MSYS2 MinGW 64-bit terminal from the start menu. You
   should build from this terminal.

3. Install the required tools.

   ```sh
   $ pacman -S pacman-mirrors
   
   $ pacman -S make diffutils mingw-w64-x86_64-gcc
   ```
