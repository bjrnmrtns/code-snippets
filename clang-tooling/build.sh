#!/bin/sh

 clang++ -Xclang -ast-dump -fsyntax-only -I./ -std=c++11 test.cpp
