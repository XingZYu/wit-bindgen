use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use std::time::Instant;

pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/memory-to-string.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("memory-to-string: {:?}", duration);

    // Test case
    let results = runtime.run("foo", &[])?;
    assert!(results[0].unwrap_string() == "foo");
    let results = runtime.run("hexa", &[])?;
    assert!(results[0].unwrap_string() == "hexa");

    // Experiment case
    start = Instant::now();
    for _ in 0..100 {
        let results = runtime.run("foo", &[])?;
        assert!(results[0].unwrap_string() == "foo");
        let results = runtime.run("hexa", &[])?;
        assert!(results[0].unwrap_string() == "hexa");
    }
    duration = start.elapsed();
    println!("m2s main: {:?}", duration);
    Ok(())
}