#!/bin/bash
# To run, make sure you're in benchmark directory
# run ./natives/run.sh
TARGET_DIR=./natives/target

mkdir -p $TARGET_DIR
command time cc -Wall -pg ./natives/fib_native.c -o $TARGET_DIR/fib_native
command time cc -Wall -pg ./natives/string_dynamic_native.c -o $TARGET_DIR/string_dynamic_native
command time cc -Wall -pg ./natives/string_static_native.c -o $TARGET_DIR/string_static_native
command time -v $TARGET_DIR/fib_native
command time -v $TARGET_DIR/string_dynamic_native
command time -v $TARGET_DIR/string_static_native 
rm -rf $TARGET_DIR
