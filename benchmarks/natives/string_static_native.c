#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char STRING[] = "Hello WebAssembly: This is a static string";

char* read() {
    return STRING;
}

int main() {
    clock_t t1, t2;
    t1 = clock();
    char *output;
    for (int i = 0; i < 100; ++i) {
        char *tmp = read();
        output = (char *)malloc(strlen(tmp));
        strcpy(output, tmp);
        free(output);
    }
    t2 = clock();
    printf("[Time Consumed]: string_static %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);
    return 0;
}