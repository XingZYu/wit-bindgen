#include <wit-bindgen.h>
#include <stdio.h>
#include <time.h>

int main() {
    clock_t t1, t2;
    wit_instance wasm_instance;
    const char wasm_file[] = "../target/benchmarks/wasm/string_static.wasm";
    t1 = clock();
    runtime_initialize(wasm_file, &wasm_instance);
    t2 = clock();
    printf("[Time Consumed] Initialization: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    wasm_val_t result;
    result.kind = WASM_STRING;

    t1 = clock();
    for (int i = 0; i < 100; ++i) 
    adapter_call(&wasm_instance, "read", NULL, &result);
    t2 = clock();
    printf("[Time Consumes] Call `read`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    printf("Reuslt is: %s\n", result.of.string);

    printf("Finished\n");
    return 0;
}
