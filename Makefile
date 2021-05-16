EXAMPLES = ./examples
WASMTIME_ROOT = ./crates/wasmtime
TARGET = ./target/examples
CC = gcc
CFLAGS = -g 

hello:
	${CC} ${EXAMPLES}/say_hello.c ${CFLAGS} \
	-I ${WASMTIME_ROOT}/crates/c-api/include \
	-I ${WASMTIME_ROOT}/crates/c-api/wasm-c-api/include \
	${WASMTIME_ROOT}/target/release/libwasmtime.a \
	-lpthread -ldl -lm \
	-o ${TARGET}/say_hello 

clean:
	rm -rf ${TARGET}
