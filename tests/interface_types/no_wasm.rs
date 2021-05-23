use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use wasmtime::Val;

pub fn test() -> Result<(), Error> {
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/no-wasm.wasm")
    )?;
    runtime.run("nop", &[])?;
    let params = [Val::S32(1)];
    let results = runtime.run("roundtrip", &params)?;
    assert!(results[0].unwrap_s32() == 1);
    Ok(())
}