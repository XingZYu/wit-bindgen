use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;

pub fn test() -> Result<(), Error> {
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/memory-to-string.wasm")
    )?;
    let results = runtime.run("foo", &[])?;
    assert!(results[0].unwrap_string() == "foo");
    let results = runtime.run("hexa", &[])?;
    assert!(results[0].unwrap_string() == "hexa");
    Ok(())
}