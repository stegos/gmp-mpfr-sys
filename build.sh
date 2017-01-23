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

# will unpack tars if UNPACK is yes
UNPACK=no
# will build libaries if BUILD is yes, useful for testing
BUILD=no

# end of variables

if [ "$UNPACK" == yes ]; then
	tar xf "$GMPTAR"
	mv gmp-$GMPVER gmp-$GMPVER-slim
	cd gmp-$GMPVER-slim
	rm -r ac*.m4 ChangeLog configure.ac demos doc
	sed -i.rm~ '/Configs for demos/,/Create config.m4/{//!d}' configure
	sed -i.rm~ '/ac_config_files=/s/[^ ]*\(doc\|demos\)[^ ]*.Makefile//g' \
	    configure
	sed -i.rm~ '/SUBDIRS = /s/doc\|demos//g' Makefile.in
	cd ..

	tar xf "$MPFRTAR"
	mv mpfr-$MPFRVER mpfr-$MPFRVER-slim
	cd mpfr-$MPFRVER-slim
	rm -r ac*.m4 ChangeLog configure.ac doc m4
	sed -i.rm~ '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
	sed -i.rm~ '/SUBDIRS = /s/doc//g' Makefile.in
	cd ..

	tar xf "$MPCTAR"
	mv mpc-$MPCVER mpc-$MPCVER-slim
	cd mpc-$MPCVER-slim
	rm -rf ac*.m4 ChangeLog configure.ac doc m4
	sed -i.rm~ '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
	sed -i.rm~ '/SUBDIRS = /s/doc//g' Makefile.in
	chmod u+w *
	cd ..

	find *-slim -name Makefile.am | xargs rm
	for m in $(find *-slim -name Makefile.in); do
		sed -i.rm~ '/Makefile:/,/esac/d' $m
	done
	find *-slim -name \*.rm~ | xargs rm
fi

if [ "$BUILD" == yes ]; then
	ln -s gmp-$GMPVER-slim gmp-src
	mkdir gmp-build
	cd gmp-build
	../gmp-src/configure --enable-fat --disable-shared --with-pic
	make -j $JOBS
	make -j $JOBS check
	cd ..

	ln -s mpfr-$MPFRVER-slim mpfr-src
	mkdir mpfr-build
	cd mpfr-build
	ln -s ../gmp-build .
	../mpfr-src/configure --enable-thread-safe --disable-shared \
			      --with-gmp-build=../gmp-build --with-pic
	make -j $JOBS
	make -j $JOBS check
	cd ..

	ln -s mpc-$MPCVER-slim mpc-src
	mkdir mpc-build
	cd mpc-build
	ln -s ../mpfr-src ../mpfr-build ../gmp-build .
	../mpc-src/configure --disable-shared \
			     --with-mpfr-include=../mpfr-src/src \
			     --with-mpfr-lib=../mpfr-build/src/.libs \
			     --with-gmp-include=../gmp-build \
			     --with-gmp-lib=../gmp-build/.libs --with-pic
	make -j $JOBS
	make -j $JOBS check
	cd ..
fi
