use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;

#[allow(unused_variables)]
pub fn test() -> Result<(), Error> {
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/empty.wasm")
    )?;
    Ok(())
}