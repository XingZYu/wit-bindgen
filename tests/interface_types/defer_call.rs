use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use std::time::Instant;

pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/defer-call.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("defer-call {:?},", duration);

    // Test case
    let results = runtime.run("foo", &[])?;
    assert!(results[0].unwrap_s32() == 0);
    let results = runtime.run("get", &[])?;
    assert!(results[0].unwrap_s32() == 1);
    
    // Experiment case
    start = Instant::now();
    for _ in 0..100 {
        runtime.run("foo", &[])?;
        runtime.run("get", &[])?;
    }
    duration = start.elapsed();
    println!("defer-call main {:?}", duration);
    Ok(())
}