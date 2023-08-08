#!/usr/bin/env bash
rm -r ./target/*
# mkdir ./lib/
# cp ../target/release/*.a ./lib/

gcc -o ./target/test ./src/main.c -ltest_utils -lcalc_test_a -L../target/release/ -L../clib/target/a/
