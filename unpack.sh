#!/bin/bash

# Copyright © 2017–2019 University of Malta

# Copying and distribution of this file, with or without modification,
# are permitted in any medium without royalty provided the copyright
# notice and this notice are preserved. This file is offered as-is,
# without any warranty.

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
MPFRVERP="$MPFRVER-p14"
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"
MPFRPATCH="$TARDIR/mpfr-$MPFRVERP-allpatches"

MPCVER=1.1.0
MPCVERP="$MPCVER"
MPCTAR="$TARDIR/mpc-$MPCVER.tar.gz"
MPCPATCH="$TARDIR/mpc-$MPCVERP-allpatches"

CHANGELOG_CHARS=100000

function truncate {
	mv "$1" "$1.rm~"
	(
		if (($2 > 0)); then
			head -c $(($2 - 1))
			head -n 1
		fi
		if [ $(head -c 1 | wc -c) == 1 ]; then
			echo "... (truncated)"
		fi
	) < "$1.rm~" > "$1"
}

# GMP
# 1. Truncate ChangeLog
# 2. Remove doc/*.info*, doc/*.tex
# 3a. Remove demos section in configure
# 3b. Remove doc/Makefile, demos/{,*/}Makefile from ac_config_files in configure
# 4. Remove doc and demos from SUBDIRS in Makefile.in
tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVERP-c
cd gmp-$GMPVERP-c
if [ -f "$GMPPATCH" ]; then
    patch -N -Z -p1 < "$GMPPATCH" > /dev/null
fi
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ -e '
/Configs for demos/,/Create config.m4/{
         /Create config.m4/!s/^/#gmp-mpfr-sys /
         s/\(#gmp-mpfr-sys\) $/\1/
}
/^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(\(doc\|demos[/a-z]*\)/Makefile\)\([^#]*\)\($\| #\),\1\4 #gmp-mpfr-sys \2\5,
        t repeat
}
' configure
sed -i.rm~ -e '
/^SUBDIRS = /{
	:repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc\|demos\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
}
' Makefile.in
cd ..

# MPFR
# 1. Truncate ChangeLog
# 2. Remove doc/*.info*, doc/*.tex
# 3. Remove doc/Makefile, mpfr.pc from ac_config_files in configure
# 4a. Remove doc from SUBDIRS in Makefile.in
# 4b. Remove $(pkgconfig_DATA) from DATA in Makefile.in
# 5. Remove get_patches.c rule in src/Makefile.in
# 6. Generate src/get_patches.c
tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVERP-c
cd mpfr-$MPFRVERP-c
if [ -f "$MPFRPATCH" ]; then
    patch -N -Z -p1 < "$MPFRPATCH" > /dev/null
fi
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ -e '
/^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc/Makefile\|mpfr.pc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
}
' configure
sed -i.rm~ -e '
/^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
/^DATA = /s,^\([^#]*\) \(\$(pkgconfig_DATA)\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
sed -i.rm~ '/get_patches.c:/,/^$/s/^\(.\)/#gmp-mpfr-sys \1/' src/Makefile.in
tools/get_patches.sh > src/get_patches.c
cd ..

# MPC
# 1. Make sure all files are user writeable
# 2. Truncate ChangeLog
# 3. Remove doc/*.info*, doc/*.tex
# 4. Remove doc/Makefile from ac_config_files in configure
# 5. Remove doc from SUBDIRS in Makefile.in
tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVERP-c
cd mpc-$MPCVERP-c
if [ -f "$MPCPATCH" ]; then
    patch -N -Z -p1 < "$MPCPATCH" > /dev/null
fi
chmod -R u+w *
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ '
/^ac_config_files=/s,^\([^#]*\) \(doc/Makefile\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' configure
sed -i.rm~ '
/^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
cd ..

# Finally
# 1. Comment Makefile:...esac sections from all Makefile.in
# 2. Remove all .rm~ files left over by sed
for m in $(find *-c -name Makefile.in); do
    sed -i.rm~ '/Makefile:/,/esac/s/^/#gmp-mpfr-sys /' $m
done
find *-c -name \*.rm~ -delete
