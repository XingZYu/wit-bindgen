#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

char *STRING;

void write(char *string) {
    if (STRING) free(STRING);
    STRING = (char *) malloc(strlen(string));
    strcpy(STRING, string);
}

char* read() {
    return STRING;
}

int main() {
    clock_t t1, t2;
    t1 = clock();
    char input[] = "Hello WebAssembly: This is a static string";
    char *output;
    for (int i = 0; i < 100; ++i) {
        write(input);
        char *tmp = read();
        output = (char *) malloc(strlen(tmp));
        strcpy(output, tmp);
        free(output);
    }
    t2 = clock();
    printf("[Time Consumed]: string_dynamic %f s\n", ((double)t2 - t1)/CLOCKS_PER_SEC);
    return 0;
}