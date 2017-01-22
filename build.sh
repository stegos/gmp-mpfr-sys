#!/bin/sh
TARDIR=~/Downloads
tar xf $TARDIR/gmp-6.1.2.tar.lz
mv gmp-6.1.2 gmp-6.1.2-slim
(
	cd gmp-6.1.2-slim
	rm -r ac*.m4 ChangeLog configure.ac demos doc Makefile.am
	sed -i '/Configs for demos/,/Create config.m4/{//!d}' configure
	sed -i '/ac_config_files=/s/[^ ]*\(doc\|demos\)[^ ]*.Makefile//g' \
	    configure
	sed -i '/Makefile:/,/esac/d' Makefile.in
	sed -i '/SUBDIRS = /s/doc\|demos//g' Makefile.in
)
tar xf $TARDIR/mpfr-3.1.5.tar.xz
mv mpfr-3.1.5 mpfr-3.1.5-slim
(
	cd mpfr-3.1.5-slim
	rm -r ac*.m4 ChangeLog configure.ac doc m4 Makefile.am
	sed -i '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
	sed -i '/Makefile:/,/esac/d' Makefile.in
	sed -i '/SUBDIRS = /s/doc//g' Makefile.in
)
tar xf $TARDIR/mpc-1.0.3.tar.gz
mv mpc-1.0.3 mpc-1.0.3-slim
(
	cd mpc-1.0.3-slim
	rm -rf ac*.m4 ChangeLog configure.ac doc m4 Makefile.am
	sed -i '/ac_config_files=/s/[^ ]*doc[^ ]*.Makefile//g' configure
	sed -i '/Makefile:/,/esac/d' Makefile.in
	sed -i '/SUBDIRS = /s/doc//g' Makefile.in
)

#do not actually compile
exit

JOBS=12
ln -s gmp-6.1.2-slim gmp-src
mkdir gmp-build
(
	cd gmp-build
	../gmp-src/configure --enable-fat --disable-shared --with-pic
	make -j $JOBS
	make -j $JOBS check
)
ln -s mpfr-3.1.5-slim mpfr-src
mkdir mpfr-build
(
	cd mpfr-build
	ln -s ../gmp-build .
	../mpfr-src/configure --enable-thread-safe --disable-shared \
			      --with-gmp-build=../gmp-build --with-pic
	make -j $JOBS
	make -j $JOBS check
)
ln -s mpc-1.0.3-slim mpc-src
mkdir mpc-build
(
	cd mpc-build
	ln -s ../mpfr-src ../mpfr-build ../gmp-build .
	../mpc-src/configure --disable-shared \
			     --with-mpfr-include=../mpfr-src/src \
			     --with-mpfr-lib=../mpfr-build/src/.libs \
			     --with-gmp-include=../gmp-build \
			     --with-gmp-lib=../gmp-build/.libs --with-pic
	make -j $JOBS
	make -j $JOBS check
)
