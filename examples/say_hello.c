#include "include/say_hello.h"

int main() {
    initialize();
    char hello[] = "Hello World";
    hello_set(hello);
    printf("%s", hello_get());
    return 0;
}
