# Rust low-level bindings for GMP, MPFR and MPC

The `gmp-mpfr-sys` crate provides Rust low-level bindings for

  - the
    [GNU Multiple Precision Arithmetic Library](https://gmplib.org/)
    (GMP),

  - the [GNU MPFR Library](http://www.mpfr.org/), a library for
    multiple-precision floating-point computations, and
	
  - [GNU MPC](http://www.multiprecision.org/), a library for the
    arithmetic of complex numbers with arbitrarily high precision.

The source of the three libraries is included in the package.

## License

This crate is free software: you can redistribute it and/or modify it
under the terms of the GNU Lesser General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
  
See [LICENSE-LGPL](LICENSE-LGPL.md) and [LICENSE-GPL](LICENSE-GPL.md)
for details.

## Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`
and `make` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc make
```

## Building on macOS

To build on macOS, you need the command-line developer tools. An easy
way to install them is to start building the crate using `cargo
build`. If the tools are not installed yet, a popup should appear
which should help you install them.

## Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below, you can follow similar steps for a 32-bit environment by
substituting 32 for 64. To install MSYS2:

1. Install MSYS2 using the [installer](https://msys2.github.io/).

2. Launch the MSYS2 MinGW 64-bit terminal from the start menu.

3. Install the required tools.

   ```sh
   pacman -S pacman-mirrors
   pacman -S diffutils make mingw-w64-x86_64-gcc
   ```
   
Then, to build a crate with a dependency on this crate:

1. Launch the MSYS MinGW 64-bit terminal from the start menu.

2. Change to the crate directory.

3. Build the crate using `cargo`.
