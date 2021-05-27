WASMTIME_ROOT = ../wasmtime
CC = gcc
C_FLAGS = -c
WASMTIME_INCLUDE = $(WASMTIME_ROOT)/crates/c-api/include
WASM_C_API_INCLUDE = $(WASMTIME_ROOT)/crates/c-api/wasm-c-api/include
WIT_BINDGEN_INCLUDE = ./include
WASMTIME_LIB = $(WASMTIME_ROOT)/target/release/libwasmtime.a
TARGET_DIR = ./target

c-api: wit-bindgen.o libwasmtime.a
	cp $(WASMTIME_INCLUDE)/wasmtime.h $(WIT_BINDGEN_INCLUDE)
	cp $(WASM_C_API_INCLUDE)/wasm.h $(WIT_BINDGEN_INCLUDE)
	cp $(WASMTIME_LIB) $(TARGET_DIR)
	cd $(TARGET_DIR) && \
	ar rc libwitbindgen.a wit-bindgen.o

libwasmtime.a:
	@echo "Building wasmtime-c-api" 
	cd $(WASMTIME_ROOT) && \
	cargo build --release -p wasmtime-c-api

wit-bindgen.o:
	mkdir -p $(TARGET_DIR)
	$(CC) $(C_FLAGS) wit-bindgen.c \
	-I $(WASMTIME_INCLUDE) -I $(WASM_C_API_INCLUDE) -I $(WIT_BINDGEN_INCLUDE) \
	-lpthread -ldl -lm \
	-o $(TARGET_DIR)/wit-bindgen.o

clean:
	rm -f $(TARGET_DIR)/*