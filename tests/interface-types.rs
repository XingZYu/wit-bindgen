mod interface_types;
use anyhow::{bail, Result, Error};
use rayon::prelude::*;
use std::env;
use std::path::Path;
use wasm_bindgen_cli_support::Bindgen;

#[test]
fn interface_types_test() -> Result<()> {
    let mut tests = Vec::new();
    let mut dir = env::current_dir()?;
    dir.push("tests");
    dir.push("interface_types");
    for entry in dir
        .read_dir()
        .expect(format!("read dir {:#?} failed", dir).as_str())
    {
        let path = entry?.path();
        let ext = path.extension().and_then(|s| s.to_str());

        if ext != Some("wat") {
            continue;
        }
        tests.push(path);
    }
    tests.sort();

    let errs = tests
        .par_iter()
        .filter_map(|t| runtest(t).err().map(|e| (t, e)))
        .collect::<Vec<_>>();

    if errs.len() == 0 {
        println!("{} tests passed", tests.len());
        return Ok(());
    }
    eprintln!("failed tests:\n");
    for (test, err) in errs {
        eprintln!("{} failure\n{}", test.display(), tab(&format!("{:?}", err)));
    }
    bail!("tests failed");
}

fn runtest(test: &Path) -> Result<(), Error> {
    let submod = match test.file_stem() {
        Some(s) => s,
        None => panic!("Empty Filename!"),
    };
    env::set_var("WASM_INTERFACE_TYPES", "1");
    let mut b = Bindgen::new();

    b.input_path(&test.to_path_buf());

    b.generate(&test.parent().unwrap())?;
    match submod.to_str().unwrap() {
        "defer-call" => interface_types::defer_call::test()?,
        "empty" => interface_types::empty::test()?,
        "integers" => interface_types::integers::test()?,
        "memory-to-string" => interface_types::memory_to_string::test()?,
        "no-wasm" => interface_types::no_wasm::test()?,
        "string-to-memory" => interface_types::string_to_memory::test()?,
        _ => (),
    };
    Ok(())
}

fn tab(s: &str) -> String {
    format!("    {}", s.replace("\n", "\n    "))
}
