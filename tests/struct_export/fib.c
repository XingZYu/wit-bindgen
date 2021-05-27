#include <wit-bindgen.h>
#include <Fib.h>
#include <stdio.h>
#include <time.h>

int main() {
    clock_t t1, t2;
    wit_instance wasm_instance;
    const char wasm_file[] = "./output/fib_wit.wasm";
    t1 = clock();
    runtime_init(&wasm_file);
    t2 = clock();
    printf("[Time Consumed] Initialization: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    t1 = clock();
    Fib fib_instance = Fib_new(&wasm_instance, "fib_new", NULL, &class_ptr);
    t2 = clock();
    printf("[Time Consumes] Call `fib_new`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    t1 = clock();
    int x = fibonacci(&fib_instance, 5);
    t2 = clock();
    printf("[Time Consumes] Call `fib_fibonacci`: %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);

    printf("Reuslt is: %d\n", x);
    printf("Finished\n");
    return 0;
}
