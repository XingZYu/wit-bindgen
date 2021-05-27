#include <wit-bindgen.h>
#include <stdio.h>
#include <time.h>

int main() {
    clock_t t1, t2;
    wit_instance wasm_instance;
    const char wasm_file[] = "../target/benchmarks/wasm/fib.wasm";
    t1 = clock();
    runtime_initialize(wasm_file, &wasm_instance);
    t2 = clock();
    printf("[Time Consumed] Initialization: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);
    wasm_val_t arg;
    arg.kind = WASM_S32;
    arg.of.i32 = 5;
    wasm_val_t result;
    result.kind = WASM_I32;

    t1 = clock();
    for (int j = 1; j <= 100; ++j) {
        for (int i = 1; i <= 30; ++i) {
            arg.of.i32 = i;
            adapter_call(&wasm_instance, "fibonacci", &arg, &result);
        }
    }
    t2 = clock();
    printf("[Time Consumes] Call `fibonacci`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    printf("Reuslt is: %d\n", result.of.i32);
    printf("Finished\n");
    return 0;
}
