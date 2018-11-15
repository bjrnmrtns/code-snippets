#!/bin/sh
red=`tput setaf 1`
nocolor=`tput sgr0`
gcc -O0 -std=c99 -g -c -o simple.o simple.c
echo "${red}objdump -dr simple.o${nocolor}"
objdump -dr simple.o
echo "${red}nm simple.o${nocolor}"
nm simple.o
rm simple.o

