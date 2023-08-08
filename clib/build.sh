#!/usr/bin/env bash
rm -r ./target/*
mkdir ./target/o/
mkdir ./target/a/

gcc -c ./src/test_utils.c -o ./target/o/test_utils.o
ar rcs ./target/a/libtest_utils.a ./target/o/test_utils.o
