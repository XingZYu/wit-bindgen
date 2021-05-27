#!/bin/bash

# Shell script to run c examples
# Usage: ./examples/run_example.sh example_name

EXAMPLE=$1
C_FILE=$EXAMPLE.c
C_DIR=./examples
WASMTIME_ROOT=./crates/wasmtime
LIB_ROOT=./crates/c-api/target

cc $C_DIR/$C_FILE -g \
    -I ./crates/c-api/include \
    $LIB_ROOT/libwitbindgen.a \
    $LIB_ROOT/libwasmtime.a \
    -lpthread -ldl -lm \
    -o $EXAMPLE && \
./$EXAMPLE

rm ./$EXAMPLE