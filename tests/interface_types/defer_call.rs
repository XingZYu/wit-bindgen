use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;

pub fn test() -> Result<(), Error> {
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/defer-call.wasm")
    )?;
    let results = runtime.run("foo", &[])?;
    assert!(results[0].unwrap_s32() == 0);
    let results = runtime.run("get", &[])?;
    assert!(results[0].unwrap_s32() == 1);
    Ok(())
}