#!/bin/bash

# This script untars gmp, mpfr, mpc and slims them down.

set -e

# Change the variables below before running

# library versions and tar locations
TARDIR="$HOME/Downloads"
GMPVER=6.1.2
GMPTAR="$TARDIR/gmp-$GMPVER.tar.lz"
MPFRVER=3.1.5
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"
MPCVER=1.0.3
MPCTAR="$TARDIR/mpc-$MPCVER.tar.gz"

# concurrent jobs for make
JOBS=12

tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVER-slim
cd gmp-$GMPVER-slim
rm -r ac*.m4 ChangeLog configure.ac demos
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/Configs for demos/,/Create config.m4/{//!d}' configure
sed -i.rm~ '/ac_config_files=/s/[^ ]*\(doc\|demos\)[^ ]*.Makefile//g' \
    configure
sed -i.rm~ '/SUBDIRS = /s/doc\|demos//g' Makefile.in
cd ..

tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVER-slim
cd mpfr-$MPFRVER-slim
rm -r ac*.m4 ChangeLog configure.ac m4
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
sed -i.rm~ '/SUBDIRS = /s/doc//g' Makefile.in
cd ..

tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVER-slim
cd mpc-$MPCVER-slim
rm -rf ac*.m4 ChangeLog configure.ac m4
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
sed -i.rm~ '/SUBDIRS = /s/doc//g' Makefile.in
chmod u+w *
cd ..

find *-slim -name Makefile.am | xargs rm
for m in $(find *-slim -name Makefile.in); do
	sed -i.rm~ '/Makefile:/,/esac/d' $m
done
find *-slim -name \*.rm~ | xargs rm
