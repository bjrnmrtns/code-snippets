#!/bin/sh
gcc -O0 -std=c99 -g -c -o simple.o simple.c
objdump -dr simple.o
rm simple.o
