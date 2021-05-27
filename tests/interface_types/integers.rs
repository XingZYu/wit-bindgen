use anyhow::{Result, Error};
use std::path::Path;
use wit_api::runtime::*;
use wasmtime::Val;
use std::time::Instant;

pub fn test() -> Result<(), Error> {
    let mut start = Instant::now();
    let runtime = Wasmruntime::runtime_initialize(
        Path::new("./tests/interface_types/integers.wasm")
    )?;
    let mut duration = start.elapsed();
    println!("integers: {:?}", duration);

    // Test case
    let params = [Val::S8(0), Val::S8(1)];
    start = Instant::now();
    let results = runtime.run("add_i8", &params)?;
    assert!(results[0].unwrap_s8() == 1);
    let params = [Val::U8(0), Val::U8(1)];
    let results = runtime.run("add_u8", &params)?;
    assert!(results[0].unwrap_u8() == 1);
    let params = [Val::S16(0), Val::S16(1)];
    let results = runtime.run("add_i16", &params)?;
    assert!(results[0].unwrap_s16() == 1);
    let params = [Val::U16(0), Val::U16(1)];
    let results = runtime.run("add_u16", &params)?;
    assert!(results[0].unwrap_u16() == 1);
    let params = [Val::S32(0), Val::S32(1)];
    let results = runtime.run("add_i32", &params)?;
    assert!(results[0].unwrap_s32() == 1);
    let params = [Val::U32(0), Val::U32(1)];
    let results = runtime.run("add_u32", &params)?;
    assert!(results[0].unwrap_u32() == 1);

    // Experiment case
    start = Instant::now();
    for _ in 0..100 {
        let params = [Val::S8(0), Val::S8(1)];
        let results = runtime.run("add_i8", &params)?;
        let params = [Val::U8(0), Val::U8(1)];
        let results = runtime.run("add_u8", &params)?;
        let params = [Val::S16(0), Val::S16(1)];
        let results = runtime.run("add_i16", &params)?;
        let params = [Val::U16(0), Val::U16(1)];
        let results = runtime.run("add_u16", &params)?;
        let params = [Val::S32(0), Val::S32(1)];
        let results = runtime.run("add_i32", &params)?;
        let params = [Val::U32(0), Val::U32(1)];
        let results = runtime.run("add_u32", &params)?;
    }
    duration = start.elapsed();
    println!("integers main: {:?}", duration);

    Ok(())
}