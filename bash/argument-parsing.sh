#!/bin/bash

for i in "$@"
do
    case $i in
        -r=*|--relpath=*)
        RELPATH="${i#*=}"
        shift
        ;;
    esac
done

echo "RELATIVE PATH = ${RELPATH}"
