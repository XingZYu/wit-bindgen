use wasm_bindgen_cli_support::Bindgen;
use anyhow::{Error, Result};
use std::path::{Path, PathBuf};
use std::env;

pub fn generate_wit_binary(source: &PathBuf, target: &Path) -> Result<(), Error> {
    env::set_var("WASM_INTERFACE_TYPES", "1");
    let mut b = Bindgen::new();

    b.input_path(&source);

    b.generate(&target)?;

    Ok(())
}