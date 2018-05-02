#!/bin/bash

# This script untars gmp, mpfr, mpc and tweaks them a little.

set -e

# Change the variables below before running

# library versions and tar locations
TARDIR="$HOME/Downloads"

GMPVER=6.1.2
GMPVERP="$GMPVER"
GMPTAR="$TARDIR/gmp-$GMPVER.tar.lz"
GMPPATCH="$TARDIR/gmp-$GMPVERP-allpatches"

MPFRVER=4.0.1
MPFRVERP="$MPFRVER-p6"
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"
MPFRPATCH="$TARDIR/mpfr-$MPFRVERP-allpatches"

MPCVER=1.1.0
MPCVERP="$MPCVER"
MPCTAR="$TARDIR/mpc-$MPCVER.tar.gz"
MPCPATCH="$TARDIR/mpc-$MPCVERP-allpatches"

# GMP
# 1. Remove ChangeLog, doc/*.info*, doc/*.tex
# 2. Remove demos section in configure
# 3. Remove doc/Makefile, demos/{,*/}Makefile from ac_config_files in configure
# 4. Remove doc and demos from SUBDIRS in Makefile.in
tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVERP-c
cd gmp-$GMPVERP-c
if [ -f "$GMPPATCH" ]; then
    patch -N -Z -p1 < "$GMPPATCH" > /dev/null
fi
rm ChangeLog doc/*.info* doc/*.tex
sed -i.rm~ -e '/Configs for demos/,/Create config.m4/{/Create config.m4/!d}' \
    configure
sed -i.rm~ -e '/^ac_config_files=/s, \(doc\|demos[/a-z]*\)/Makefile,,g' \
    configure
sed -i.rm~ -e '/^SUBDIRS = /s/ doc\| demos//g' Makefile.in
cd ..

# MPFR
# 1. Remove ChangeLog, doc/*.info*, doc/*.tex
# 2. Remove doc/Makefile, mpfr.pc from ac_config_files in configure
# 3. Remove doc from SUBDIRS in Makefile.in
# 4. Remove $(pkgconfig_DATA) from DATA in Makefile.in
tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVERP-c
cd mpfr-$MPFRVERP-c
if [ -f "$MPFRPATCH" ]; then
    patch -N -Z -p1 < "$MPFRPATCH" > /dev/null
fi
rm ChangeLog doc/*.info* doc/*.tex
sed -i.rm~ -e '/^ac_config_files=/s,\( doc/Makefile\| mpfr.pc\),,g' configure
sed -i.rm~ '/^SUBDIRS = /s/ doc//g' Makefile.in
sed -i.rm~ '/^DATA = /s/ \$(pkgconfig_DATA)//g' Makefile.in
cd ..

# MPC
# 1. Remove ChangeLog, doc/*.info*, doc/*.tex
# 2. Remove doc/Makefile from ac_config_files in configure
# 3. Remove doc from SUBDIRS in Makefile.in
tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVERP-c
cd mpc-$MPCVERP-c
if [ -f "$MPCPATCH" ]; then
    patch -N -Z -p1 < "$MPCPATCH" > /dev/null
fi
chmod -R u+w *
rm ChangeLog doc/*.info* doc/*.tex
sed -i.rm~ '/^ac_config_files=/s, doc/Makefile,,g' configure
sed -i.rm~ '/^SUBDIRS = /s/ doc//g' Makefile.in
cd ..

# Finally
# 1. Comment Makefile:...esac sections from all Makefile.in
# 2. Remove all .rm~ files left over by sed
for m in $(find *-c -name Makefile.in); do
    sed -i.rm~ '/Makefile:/,/esac/s/^/# /' $m
done
find *-c -name \*.rm~ -delete
