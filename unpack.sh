#!/bin/bash

# This script untars gmp, mpfr, mpc and slims them down.

set -e

# Change the variables below before running

# library versions and tar locations
TARDIR="$HOME/Downloads"

GMPVER=6.1.2
GMPVERP="$GMPVER"
GMPTAR="$TARDIR/gmp-$GMPVER.tar.lz"
GMPPATCH="$TARDIR/gmp-$GMPVERP-allpatches"

MPFRVER=4.0.0
MPFRVERP="$MPFRVER"
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"
MPFRPATCH="$TARDIR/mpfr-$MPFRVERP-allpatches"

MPCVER=1.1-rc1
MPCVERP="$MPCVER"
MPCTAR="$TARDIR/mpc-$MPCVER.tar.gz"
MPCPATCH="$TARDIR/mpc-$MPCVERP-allpatches"

tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVERP-slim
cd gmp-$GMPVERP-slim
if [ -f "$GMPPATCH" ]; then
	patch -N -Z -p1 < "$GMPPATCH" > /dev/null
fi
rm -r ac*.m4 ChangeLog configure.ac demos
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/Configs for demos/,/Create config.m4/{//!d}' configure
sed -i.rm~ '/^ac_config_files=/s/[^ ]*\(doc\|demos\)[^ ]*.Makefile//g' \
    configure
sed -i.rm~ '/^SUBDIRS = /s/doc\|demos//g' Makefile.in
cd ..

tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVERP-slim
cd mpfr-$MPFRVERP-slim
if [ -f "$MPFRPATCH" ]; then
	patch -N -Z -p1 < "$MPFRPATCH" > /dev/null
fi
rm -r ac*.m4 ChangeLog configure.ac m4
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/^ac_config_files=/s/\([^ ]*doc[^ ]*.Makefile\|mpfr.pc\)//g' configure
sed -i.rm~ '/^SUBDIRS = /s/doc//g' Makefile.in
sed -i.rm~ '/^DATA = /s/\$(pkgconfig_DATA)//g' Makefile.in
cd ..

tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVERP-slim
cd mpc-$MPCVERP-slim
if [ -f "$MPCPATCH" ]; then
	patch -N -Z -p1 < "$MPCPATCH" > /dev/null
fi
rm -rf ac*.m4 ChangeLog configure.ac m4
find doc -name \*.tex\* -o -type f -delete
sed -i.rm~ '/^ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
sed -i.rm~ '/^SUBDIRS = /s/doc//g' Makefile.in
chmod u+w * doc/*
cd ..

find *-slim -name Makefile.am | xargs rm
for m in $(find *-slim -name Makefile.in); do
	sed -i.rm~ '/Makefile:/,/esac/d' $m
done
find *-slim -name \*.rm~ | xargs rm
