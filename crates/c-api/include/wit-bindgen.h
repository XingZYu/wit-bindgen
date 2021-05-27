#ifndef WIT_BINDGEN_API_H
#define WIT_BINDGEN_API_H

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <wasm.h>
#include <wasmtime.h>

typedef struct wit_instance {
    wasm_instance_t *instance;
    wasm_module_t *module_;
    wasm_engine_t *engine;
    wasm_name_t *exports;
    wasm_store_t *store;
} wit_instance;

void runtime_initialize(const char *, wit_instance *);
static void print_trap(wasm_trap_t *);
void adapter_call(wit_instance *, const char *, const wasm_val_t *, wasm_val_t *);
void runtime_free(wit_instance *);

#endif // WASMTIME_API_H