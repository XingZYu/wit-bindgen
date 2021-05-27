mod workflow_test;
use anyhow::{bail, Result, Error};
use rayon::prelude::*;
use std::env;
use std::path::PathBuf;
use std::ffi::OsString;
use std::process::Command;
use std::fs;
use std::io::{self, Write};
use wit_api::generate_wit_binary;

#[test]
fn workflow() -> Result<()> {
    let filter = env::args().nth(1);

    let mut source = Vec::new();
    let mut dir = env::current_dir()?;
    dir.push("tests");
    dir.push("struct_export");

    for entry in dir
        .read_dir()
        .expect(format!("read dir {:#?} failed", dir).as_str())
    {
        let path = entry?.path().to_path_buf();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str());

        if ext != Some("rs") {
            continue;
        }
        if let Some(filter) = &filter {
            if !path.display().to_string().contains(filter) {
                continue;
            }
        }
        source.push(path);
    }
    source.sort();

    let errs = source 
        .par_iter()
        .filter_map(|t| run_benchmark(t).err().map(|e| (t, e)))
        .collect::<Vec<_>>();

    if errs.len() == 0 {
        println!("{} tests passed", source.len());
        return Ok(());
    }
    eprintln!("failed tests:\n");
    for (name, err) in errs {
        eprintln!("{} failure\n{}", name.display(), tab(&format!("{}", err)));
    }
    bail!("tests failed");
}

fn compile_to_wasm(source: &mut PathBuf, debug: bool) -> Result<Option<()>, Error> {
    let mut build = Command::new("cargo");
    let mut overwirte = false;
    let feature_default: String;
    let td = tempfile::TempDir::new()?;
    let mut stem = OsString::from(source.file_stem().unwrap());
    stem.push(".wasm");

    let target = source.parent().unwrap().join(&stem);
    if debug {
        feature_default = "default = [\"xxx_debug_only_print_generated_code\"]".to_string();
    } else {
        feature_default = "".to_string();
    }
    
    let pkg_name = match source.file_stem().unwrap().to_str() {
        Some(s) => s,
        _ => "fib"
    };
    let manifest = format!(
        "
            [package]
            name = \"{}\"
            authors = []
            version = \"1.0.0\"
            edition = '2018'

            [features]
            {}
            strict-macro = [\"wasm-bindgen-macro-support/strict-macro\"]
            xxx_debug_only_print_generated_code = [\"wasm-bindgen-macro/xxx_debug_only_print_generated_code\"]
            
            [dependencies]
            wasm-bindgen = {{ path = '{}', features=['strict-macro'] }}
            wasm-bindgen-macro = {{ path = '{}' }}
            wasm-bindgen-macro-support = {{ path = '{}' }}

            [lib]
            crate-type = ['cdylib']
            path = '{}'
        ",
        pkg_name,
        feature_default,
        bindgen_root().display(),
        bindgen_root().join("crates").join("macro").display(),
        bindgen_root().join("crates").join("macro-support").display(),
        source.display(),
    );

    source.pop();
    source.push(&stem);
    if target.is_file() {
        overwirte = true;
        fs::remove_file(&target)?;
    }

    fs::write(td.path().join("Cargo.toml"), manifest)?;
    build
        .current_dir(&td.path())
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &td.path().join("target"));

    exec(&mut build)?;
    
    // println!("Target Path: {:#?}", &target);
    let binary_path = td
        .path()
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("debug")
        .join(&stem);
    fs::copy(&binary_path, &target)?;

    if overwirte {
        return Ok(None);
    } else {
        return Ok(Some(()));
    }
}

fn run_benchmark(source: &PathBuf) -> Result<(), Error> {
    let mut wasm_path = source.to_path_buf();
    compile_to_wasm(&mut wasm_path, false)?;
    // println!("Wasm Path: {:#?}", &wasm_path);
    let target = env::current_dir()?
        .join("tests")
        .join("struct_export");

    // println!("Wit Path: {:#?}", &td);

    generate_wit_binary(&wasm_path, &target)?;
    Ok(())
}

fn exec(cmd: &mut Command) -> Result<(), Error> {
    // println!("{:#?}", cmd);
    let output = cmd.output().expect("failed to execute process");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        println!(
            "Command failed: {}",
            &String::from_utf8_lossy(&output.stderr)
        );
    } 
    Ok(())
}

fn tab(s: &str) -> String {
    format!("    {}", s.replace("\n", "\n    "))
}

fn bindgen_root() -> PathBuf {
    let mut root = env::current_dir().unwrap();
    root.push("crates");
    root.push("wasm-bindgen");
    root 
}
