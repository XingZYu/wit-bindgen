BENCHMARKS = ./benchmark
WIT_BINDGEN_INCLUDE = ../crates/c-api/include
STATIC_LIBS = ../crates/c-api/target
TARGET = ../target/benchmarks
CC = gcc 
CFLAGS = -g
TESTS = fib.o string_dynamic.o string_static.o 
OBJECTS := $(patsubst %.c,%.o,${TESTS}) 

build: targetdir ${TESTS}
	@echo "Build Success"

%.o:
	time ${CC} ${BENCHMARKS}/$(patsubst %.o,%.c,$@) ${CFLAGS} \
	-I ${WIT_BINDGEN_INCLUDE} \
	${STATIC_LIBS}/libwitbindgen.a ${STATIC_LIBS}/libwasmtime.a \
	-lpthread -ldl -lm \
	-o $@
	@echo "Make Complete"

targetdir:
	mkdir -p ${TARGET}

clean:
	rm *.o
