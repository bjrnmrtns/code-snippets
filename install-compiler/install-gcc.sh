#!/bin/bash
# Prerequisites: apt install libgmp-dev libmpc-dev libmpfr-dev zlib1g-dev binutils
# Download fairly old compiler
wget http://ftp.gnu.org/gnu/gcc/gcc-4.9.4/gcc-4.9.4.tar.bz2 &&
tar xvfj gcc-4.9.4.tar.bz2 &&
cd gcc-4.9.4 &&
mkdir build &&
cd build &&
../configure --enable-languages=c,c++ --enable-shared --enable-threads=posix --program-suffix=4.9.4 --with-gmp=/usr/local/lib --with-mpc=/usr/lib --with-mpfr=/usr/lib --with-tune=generic --with-system-zlib --disable-multilib --prefix=${HOME}/install/gcc-4.9.4 &&
make -j8 &&
make install
