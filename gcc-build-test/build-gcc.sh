#!/bin/sh
wget https://mirror.lyrahosting.com/gnu/gcc/gcc-11.1.0/gcc-11.1.0.tar.xz
# apt-get install lib32z1
tar xf gcc-11.1.0.tar.xz
cd gcc-11.1.0
./contrib/download_prerequisites
cd ..
mkdir build && cd build

../gcc-11.1.0/configure -v --build=x86_64-linux-gnu --host=x86_64-linux-gnu --target=x86_64-linux-gnu --prefix=/usr/local/gcc-11.1.0 --enable-checking=release --enable-languages=c,c++,fortran --disable-multilib --program-suffix=-11.1

make -j8
sudo make -i install-strip

# add to .bashrc
#export PATH=/usr/local/gcc-11.1.0/bin:$PATH
#export LD_LIBRARY_PATH=/usr/local/gcc-11.1.0/lib64:$LD_LIBRARY_PATH
