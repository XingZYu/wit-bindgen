use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use wasmtime::Val;

#[allow(dead_code)]
fn test_str(s: &str, runtime: &Wasmruntime) -> Result<(), Error> {
    let params = [Val::String(s.to_string())];
    runtime.run("set", &params)?; 
    let results = runtime.run("get", &[])?; 
    assert!(results[0].unwrap_string() == s);
    Ok(())
}

pub fn test() -> Result<(), Error> {
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/string-to-memory.wasm")
    )?;
    test_str("", &runtime)?;
    test_str("x", &runtime)?;
    test_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &runtime)?;
    Ok(())
}