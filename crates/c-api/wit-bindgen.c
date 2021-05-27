#include <wit-bindgen.h>

static void print_trap(wasm_trap_t *trap)
{
    assert(trap != NULL);
    wasm_message_t message;
    wasm_trap_message(trap, &message);
    fprintf(stderr, "failed to instantiate module %.*s\n", (int)message.size, message.data);
    wasm_byte_vec_delete(&message);
    wasm_trap_delete(trap);
}

void runtime_initialize(const char* source, wit_instance *target) {
    // printf("Initializing...\n");
    wasm_config_t *config = wasm_config_new();
    wasmtime_config_wasm_interface_types_set(config, true);

    wasm_engine_t *engine = wasm_engine_new_with_config(config);
    target -> engine = engine;
    assert(engine != NULL);

    wasm_store_t *store = wasm_store_new(engine);
    target -> store = store;
    assert(store != NULL);

    FILE *file = fopen(source, "r");
    assert(file != NULL);
    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    fseek(file, 0L, SEEK_SET);
    wasm_byte_vec_t wasm;
    wasm_byte_vec_new_uninitialized(&wasm, file_size);
    assert(fread(wasm.data, file_size, 1, file) == 1);
    fclose(file);

    // printf("Compiling module...\n");
    wasm_module_t *module = wasm_module_new(store, &wasm);
    target -> module_ = module;
    wasm_byte_vec_delete(&wasm);
    assert(module != NULL);

    // printf("Instantiating module...\n");
    wasm_trap_t *trap = NULL;
    const wasm_extern_t *imports = NULL;
    wasm_instance_t *instance = wasm_instance_new(store, module, NULL, &trap);
    if (instance == NULL)
    {
        print_trap(trap);
        runtime_free(target);
        printf("Initialization Failed!\n");
        return ;
    }
    target -> instance = instance;
    // printf("Initialized!\n");
}

void adapter_call(
    wit_instance *instance, 
    const char *name, 
    const wasm_val_t *args,
    wasm_val_t *results
) {
    wasm_name_t func_name;
    wasm_extern_vec_t export_func;
    wasm_trap_t *trap;
    wasm_name_new_from_string(&func_name, name);
    // printf("Extracting export %s...\n", name);
    trap = wasm_get_export(instance->instance, &func_name, &export_func);
    if (trap != NULL) print_trap(trap);

    wasm_adapter_t *adapter = wasm_extern_as_adapter(export_func.data[0]);
    // printf("Calling export %s...\n", name);
    trap = wasm_adapter_call(adapter, args, results);
    if (trap != NULL) 
    {
        print_trap(trap);
        runtime_free(instance);
    }
}

void runtime_free(wit_instance *wasm) {
    if (wasm -> instance) wasm_instance_delete(wasm->instance);
    if (wasm -> module_) wasm_module_delete(wasm->module_);
    if (wasm -> store) wasm_store_delete(wasm->store);
    if (wasm -> engine) wasm_engine_delete(wasm->engine);
}