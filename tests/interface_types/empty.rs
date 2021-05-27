use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use std::time::Instant;

#[allow(unused_variables)]
pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/empty.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("empty: {:?}", duration);
    Ok(())
}