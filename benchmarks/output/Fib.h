#include <stdlib.h>;

wit_instance wasm_instance;
bool wasm_instance_init = false;
typedef struct Fib {
	int32_t class_ptr;
} Fib;

int32_t fibonacci(Fib *class_ptr, int32_t arg_0) {
	if (!wasm_instance_init) { printf("Module Uninitialized\n"); exit(-1); }
	wasm_val_t args[2];
	wasm_val_t results[1];
	args[0].kind = WASM_I32;
	args[0].of.i32 = arg_0;
	args[1].kind = WASM_I32;
	args[1].of.i32 = arg_1;
	results[0].kind = WASM_I32;
	results[0].of.i32 = arg_0;
}

Fib Fib_new() {
	if (!wasm_instance_init) { printf("Module Uninitialized\n"); exit(-1); }
	wasm_val_t args[0];
	wasm_val_t results[1];
	results[0].kind = WASM_S32;
	results[0].of.i32 = arg_0;
}

