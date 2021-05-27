use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use wasmtime::Val;
use std::time::Instant;

#[allow(dead_code)]
fn test_str(s: &str, runtime: &Wasmruntime) -> Result<(), Error> {
    let params = [Val::String(s.to_string())];
    runtime.run("set", &params)?; 
    let results = runtime.run("get", &[])?; 
    assert!(results[0].unwrap_string() == s);
    Ok(())
}

#[allow(dead_code)]
fn test_str_not_assert(s: &str, runtime: &Wasmruntime) -> Result<(), Error> {
    let params = [Val::String(s.to_string())];
    runtime.run("set", &params)?; 
    let results = runtime.run("get", &[])?; 
    Ok(())
}

pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/string-to-memory.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("string-to-memory: {:?}", duration);

    // Test case
    test_str("", &runtime)?;
    test_str("x", &runtime)?;
    test_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &runtime)?;

    // Experiment case
    start = Instant::now();
    for _ in 0..100 {
        test_str_not_assert("", &runtime)?;
        test_str_not_assert("x", &runtime)?;
        test_str_not_assert("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &runtime)?;
    }
    duration = start.elapsed();
    println!("s2m main: {:?}", duration);
    Ok(())
}