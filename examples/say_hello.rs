use wasmtime::*;
use std::error::Error;
use anyhow::Result;


fn main() -> Result<()> {
    println!("Initializing...");
    let engine = Engine::new(Config::new()
        .wasm_interface_types(true)
        .wasm_reference_types(true)
    );
    let store = Store::new(&engine);

    // Compile the wasm binary into an in-memory instance of a `Module`.
    println!("Compiling module...");

    let module = Module::from_file(&store, "examples/struct_test.wasm")?;
    
    // println!("Module Name: {:#?}", module.name());
    // println!("Module Exports: {:#?}", module.exports());
    // println!("Module Imports: {:#?}", module.imports());
    // println!("Module Adapters: {:#?}", module.adapters());

    // After we have a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    
    println!("Instantiating module...");
    let instance = Instance::new(&module, &[])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.

    let answer = instance.get_export("hello_new")
                .and_then(|e| e.adapter())
                .ok_or(anyhow::format_err!("failed to run constructor"))?;
    let ptr = answer.call(&[])?;
    let params = [Val::S32(ptr[0].unwrap_i32())];

    match run("hello_get", &instance, &params){
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    }

    let params = [Val::S32(ptr[0].unwrap_i32()), Val::String("Hello World!".to_string())];
    match run("hello_set", &instance, &params) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {:#?}", e),
    }
    
    let params = [Val::S32(ptr[0].unwrap_i32())];
    match run("hello_get", &instance, &params) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    }

    let params = [Val::S32(ptr[0].unwrap_i32()), Val::String("Hello again!".to_string())];
    match run("hello_set", &instance, &params) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {:#?}", e),
    }
    
    let params = [Val::S32(ptr[0].unwrap_i32())];
    match run("hello_get", &instance, &params) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    }

    Ok(())
}

fn run(func_name: &str, instance: &Instance, params: &[Val]) -> Result<Vec<String>, Box<dyn Error>> {
    let answer = instance.get_export(&func_name)
        .and_then(|e| e.adapter())
        .ok_or(anyhow::format_err!("failed to find `run` function export"))?;

    println!("Calling export...");
    let res_string: Vec<String>;

    let result = answer.call(&params)?;

    println!("Call into Func '{}' succeed", func_name);
    println!("");
    if result.len() > 0 {
        res_string = result.iter()
            .map(|s| print_result(&s))
            .collect()
    }
    else {
        res_string = vec!("Empty result".to_string());
    }
    Ok(res_string)
}

fn print_result(value: &Val) -> String {
    let rust_val = match value {
        Val::I32(_) => format!("{}", value.unwrap_i32()),
        Val::String(_) => value.unwrap_string().to_string(),
        _ => unimplemented!("Not implemented types"),
    };
    println!("{}", rust_val);
    rust_val
}