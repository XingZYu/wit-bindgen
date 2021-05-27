use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use wasmtime::Val;
use std::time::Instant;

pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/no-wasm.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("no-wasm: {:?}", duration);
    
    // Test case
    runtime.run("nop", &[])?;
    let params = [Val::S32(1)];
    let results = runtime.run("roundtrip", &params)?;
    assert!(results[0].unwrap_s32() == 1);

    // Experiment case
    start = Instant::now();
    for _ in 0..100 {
        runtime.run("nop", &[])?;
        let params = [Val::S32(1)];
        let results = runtime.run("roundtrip", &params)?;
    }
    duration = start.elapsed();
    println!("nowasm main: {:?}", duration);
    Ok(())
}