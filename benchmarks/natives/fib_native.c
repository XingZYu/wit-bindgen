#include <stdint.h>
#include <time.h>
#include <stdio.h>

int32_t fibonacci(int n) {
    int32_t a = 1, b = 1;
    for (int i = 0; i < n; ++i) {
        int32_t tmp = b;
        b += a;
        a = tmp;
    }
    return a;
}

int main() {
    clock_t t1, t2;
    t1 = clock();
    int x;
    for (int j = 0; j < 100; ++j) {
        for (int i = 1; i < 30; ++i) {
            x = fibonacci(i);
        }
    }
    t2 = clock();
    printf("[Time Consumed]: fib %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);
    return 0;
}