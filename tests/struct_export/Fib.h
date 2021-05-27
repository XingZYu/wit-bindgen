#include <stdlib.h>;

wit_instance wasm_instance;
bool wasm_instance_init = false;
void runtime_init(const char *wasm_file) {
	runtime_initialize(wasm_file, &wasm_instance);
	wasm_instance_init = true;
}

typedef struct Fib {
	int32_t class_ptr;
} Fib;

Fib Fib_new() {
	if (!wasm_instance_init) { printf("Module Uninitialized\n"); exit(-1); }
	wasm_val_t results[1];
	results[0].kind = WASM_S32;
	results[0].of.i32 = arg_0;
	adapter_call(&wasm_instance, "new", &NULL, &results);
}

int32_t fibonacci(Fib *fib_ptr, int32_t arg_0) {
int32_t arg_0 = fib_ptr -> class_ptr
	if (!wasm_instance_init) { printf("Module Uninitialized\n"); exit(-1); }
	wasm_val_t args[2];
	wasm_val_t results[1];
	args[0].kind = WASM_I32;
	args[0].of.i32 = arg_0;
	args[1].kind = WASM_I32;
	args[1].of.i32 = arg_1;
	results[0].kind = WASM_I32;
	results[0].of.i32 = arg_0;
	adapter_call(&wasm_instance, "fibonacci", &args, &results);
}

