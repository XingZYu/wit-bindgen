use wasmtime::*;
use std::path::Path;
use anyhow::{Result, Error};

#[allow(dead_code)]
pub struct Wasmruntime {
    engine: Engine,
    store: Store,
    module: Module,
    instance: Instance,
}

#[allow(dead_code)]
impl Wasmruntime {
    pub fn runtime_initialize(source: &Path) -> Result<Self, Error> {
        // println!("Initializing...");
        let engine = Engine::new(Config::new()
            .wasm_interface_types(true)
            .wasm_reference_types(true)
        );
        let store = Store::new(&engine);

        // println!("Compiling module...");
        let module = Module::from_file(&store, &source)?;

        // println!("Instantiating module...");
        let instance = Instance::new(&module, &[])?;
        Ok(Wasmruntime {
            engine,
            store,
            module,
            instance,
        })
    }

    pub fn run(&self, func_name: &str, params: &[Val]) -> Result<Box<[Val]>, Error> {
        let answer = self.instance.get_export(&func_name)
            .and_then(|e| e.adapter())
            .ok_or(anyhow::format_err!("failed to find `run` function export"))?;

        // println!("Calling export...");
        
        Ok(answer.call(&params)?)
    }
}
