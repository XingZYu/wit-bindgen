#include <wit-bindgen.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

int main() {
    clock_t t1, t2;
    wit_instance wasm_instance;
    const char wasm_file[] = "../target/benchmarks/wasm/string_dynamic.wasm";
    const char input[] = "Hello WebAssembly: This is a static string";
    t1 = clock();
    runtime_initialize(wasm_file, &wasm_instance);
    t2 = clock();
    printf("[Time Consumed] Initialization: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    wasm_val_t arg;
    arg.kind = WASM_STRING;
    arg.of.string = input;

    t1 = clock();
    for (int i = 0; i < 100; ++i) 
    adapter_call(&wasm_instance, "write", &arg, NULL);
    t2 = clock();
    printf("[Time Consumes] Call `write`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    wasm_val_t result;
    result.kind = WASM_STRING;
    char *output;

    t1 = clock();
    for (int i = 0; i < 100; ++i) {
        adapter_call(&wasm_instance, "read", NULL, &result);
        output = (char *) malloc(strlen(result.of.string));
        strcpy(output, result.of.string);
        free(output);
    }
    t2 = clock();
    printf("[Time Consumes] Call `read`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    printf("Reuslt is: %s\n", output);

    printf("Finished\n");
    return 0;
}
