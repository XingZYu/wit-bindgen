#include <wit-bindgen.h>
#include <stdio.h>

int main() {
    wit_instance wasm_instance;
    const char wasm_file[] = "./examples/struct_test.wasm";
    runtime_initialize(wasm_file, &wasm_instance);

    wasm_val_t result;
    result.kind = WASM_I32;
    adapter_call(&wasm_instance, "hello_new", NULL, &result);

    printf("Finished\n");
    return 0;
}
