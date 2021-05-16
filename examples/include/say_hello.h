#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <wasm.h>
#include <wasmtime.h>

wasm_instance_t *instance;
wasm_extern_vec_t *funcs;
wasm_trap_t *trap = NULL;

static void print_trap(wasm_trap_t *trap)
{
    assert(trap != NULL);
    wasm_message_t message;
    wasm_trap_message(trap, &message);
    fprintf(stderr, "failed to instantiate module %.*s\n", (int)message.size, message.data);
    wasm_byte_vec_delete(&message);
    wasm_trap_delete(trap);
}

void wasm_free() {
    wasm_extern_vec_delete(&funcs);
    wasm_instance_delete(instance);
}

void initialize() {
    printf("Initializing...\n");
    wasm_config_t *config = wasm_config_new();
    wasmtime_config_wasm_interface_types_set(config, true);

    wasm_engine_t *engine = wasm_engine_new_with_config(config);
    assert(engine != NULL);

    wasm_store_t *store = wasm_store_new(engine);
    assert(store != NULL);

    FILE *file = fopen("examples/string-to-memory.wasm", "r");
    assert(file != NULL);
    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    fseek(file, 0L, SEEK_SET);
    wasm_byte_vec_t wasm;
    wasm_byte_vec_new_uninitialized(&wasm, file_size);
    assert(fread(wasm.data, file_size, 1, file) == 1);
    fclose(file);

    printf("Compiling module...\n");
    wasm_module_t *module = wasm_module_new(store, &wasm);
    wasm_byte_vec_delete(&wasm);
    assert(module != NULL);

    printf("Instantiating module...\n");
    wasm_trap_t *trap = NULL;
    const wasm_extern_t *imports = NULL;
    wasm_instance_t *instance = wasm_instance_new(store, module, NULL, &trap); 

    if (instance == NULL) {
        goto free_module;
    }

    printf("Extracting export...\n");
    wasm_extern_vec_t externs;
    wasm_instance_exports(instance, &externs);

    const wasm_module_t *constModule = module;
    wasm_exporttype_vec_t exports;
    wasm_module_exports(constModule, &exports);

    assert(externs.size == 2);
    funcs = malloc(exports.size*sizeof(wasm_extern_vec_t));
    for (int i = 0; i < exports.size; ++i) {
        wasm_exporttype_t *export_type = exports.data[i];
        const wasm_name_t *export_name = wasm_exporttype_name(export_type);
        wasm_extern_vec_t *func = &funcs[i];
        trap = wasm_get_export(instance, export_name, func);
        if (trap != NULL) print_trap(trap);
    }

free_module:
    wasm_module_delete(module);
free_store:
    wasm_store_delete(store);
    wasm_engine_delete(engine);
}

void hello_set(char *string) {
    wasm_adapter_t *set = wasm_extern_as_adapter(funcs[0].data[0]);
    assert(set != NULL);
    
    printf("Calling export `set`...\n");
    wasm_val_t args[1];
    args[0].kind = WASM_STRING;
    args[0].of.string = string;
    trap = wasm_adapter_call(set, args, NULL);
    if (trap != NULL)
    {
        print_trap(trap);
        wasm_free();
    }
}

char* hello_get() {
    wasm_adapter_t *get = wasm_extern_as_adapter(funcs[1].data[0]);
    assert(get != NULL);

    printf("Calling export `get`...\n");
    wasm_val_t results[1];
    trap = wasm_adapter_call(get, NULL, results);
    if (trap != NULL)
    {
        print_trap(trap);
        wasm_free();
    }
    printf("%s\n", results[0].of.string);
}
