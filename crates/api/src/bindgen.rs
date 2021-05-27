use wasm_bindgen_cli_support::Bindgen;
use wasm_bindgen_cli_support::wit::{AuxExport, AuxExportKind};
use anyhow::{Error, Result};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env;

pub fn generate_wit_binary(source: &PathBuf, target: &Path) -> Result<(), Error> {
    env::set_var("WASM_INTERFACE_TYPES", "1");
    let mut b = Bindgen::new();
    b.json_aux(true);

    b.input_path(&source);

    b.generate(&target)?;

    let mut cgen = CBuilder { structs: HashMap::new() };
    cgen.read_json(&target)?;
    cgen.struct_gen(&target)?;
    Ok(())
}

#[derive(Default)]
struct CBuilder {
    structs: HashMap<String, Vec<AuxExport>>,
}

impl CBuilder {
    pub fn struct_gen(&self, p: &Path) -> Result<(), Error> {
        for (stem, _) in self.structs.iter() {
            let mut out = String::new();
            let header_name = format!("{}.h", &stem);
            let data = match self.structs.get(&stem.to_string()) {
                Some(s) => s,
                None => panic!("Generation Failed: No relevant decription of {}", &stem),
            };
            let mut f = File::create(p.join(&header_name))?;
            out.push_str(&CBuilder::global_info()?);
            out.push_str(&format!("typedef struct {} {{\n", &stem));
            out.push_str(&format!("\tint32_t class_ptr;\n}} {};\n\n", &stem));
            for func in data.iter() {
                let func_def = CBuilder::method_gen(&func)?;
                out.push_str(&func_def);
                out.push_str("\n");
            }
            f.write_all(out.as_bytes())?;
        }
        Ok(())
    }
    
    fn func_body_gen(name: &str, params: &Vec<String>, results: &Vec<String>) 
    -> Result<String, Error> {
        let mut out = String::new();
        out.push_str("\t");
        out.push_str(&check_initialize());
        if params.len() > 0 {
            out.push_str(&format!("\twasm_val_t args[{}];\n", params.len()));
        }
        if results.len() > 0 {
            out.push_str(&format!("\twasm_val_t results[{}];\n", results.len()));
        }
        let mut arg_str = "NULL";
        let mut result_str = "NULL";
        if params.len() > 0 { arg_str = "args"; } 
        if results.len() > 0 { result_str = "results"; }

        for (i, param) in params.iter().enumerate() {
            let kind = get_kind(param);
            let wasm_kind = get_wasm_kind(param);

            out.push_str(&format!("\targs[{}].kind = {};\n", i, wasm_kind));
            out.push_str(&format!("\targs[{}].of.{} = arg_{};\n", i, kind, i));
        }

        for (i, result) in results.iter().enumerate() {
            let kind = get_kind(result);
            let wasm_kind = get_wasm_kind(result);

            out.push_str(&format!("\tresults[{}].kind = {};\n", i, wasm_kind));
            out.push_str(&format!("\tresults[{}].of.{} = arg_{};\n", i, kind, i));
        }

        
        let adapter_call = format!(
            "\tadapter_call(&wasm_instance, \"{}\", &{}, &{});\n", 
            name, 
            arg_str,
            result_str
        );
        out.push_str(&adapter_call);
        Ok(out)
    }

    fn method_gen(func: &AuxExport) -> Result<String, Error> {
        let mut out = String::new();
        match &func.kind {
            AuxExportKind::Method { class, name, .. } => {
                let (params, results) = func.signature.as_ref().unwrap();
                let class_lower = class.to_lowercase();
                let mut decl = format!(
                    "{} {}({} *{}_ptr", 
                    results[0], 
                    name, 
                    class, 
                    class_lower
                );
                for (i, type_name) in params[1..].iter().enumerate() {
                    decl.push_str(&format!(", {} arg_{}", type_name, i))
                }
                decl.push_str(") {\n");
                let ptr_to_int = format!("\tint32_t arg_0 = {}_ptr -> class_ptr\n", class_lower);
                decl.push_str(&ptr_to_int);
                let contents = CBuilder::func_body_gen(&name, params, results)?;
                decl.push_str(&contents);
                decl.push_str("}\n");
                out.push_str(&decl);
            },
            AuxExportKind::StaticFunction { class, name, .. } => {
                let static_name = format!("{}_{}", class, name);
                let (params, results) = func.signature.as_ref().unwrap();
                let mut decl = format!("{} {}(", results[0], static_name);
                for type_name in params.iter() {
                    decl.push_str(&format!(", {}", type_name))
                }
                decl.push_str(") {\n");
                let contents = CBuilder::func_body_gen(&name, params, results)?;
                decl.push_str(&contents);
                decl.push_str("}\n");
                out.push_str(&decl);
            }
            _ => (),
        }
        Ok(out)
    }

    pub fn read_json(&mut self, p: &Path) -> Result<(), Error> {
        for entry in p 
            .read_dir()
            .expect(format!("read dir {:#?} failed", p).as_str())
        {
            let path = entry?.path().to_path_buf();
            let ext = path.extension().and_then(|s| s.to_str());

            if ext != Some("json") {
                continue;
            }
            // Open the file in read-only mode with buffer.
            let mut file = File::open(path)?;
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            // Read the JSON contents of the file as an instance of `AuxExport`.
            self.structs = serde_json::from_str(&data)?;
            // println!("{:#?}", &self.structs);
            // std::fs::remove_file(&p)?;
        }
        Ok(())
    }

    fn global_info() -> Result<String, Error> {
        let mut out = String::new();
        out.push_str("#include <stdlib.h>;\n\n");
        out.push_str("wit_instance wasm_instance;\n");
        out.push_str("bool wasm_instance_init = false;\n");
        out.push_str("void runtime_init(const char *wasm_file) {\n");
        out.push_str("\truntime_initialize(wasm_file, &wasm_instance);\n");
        out.push_str("\twasm_instance_init = true;\n");
        out.push_str("}\n\n");
        Ok(out)
    }
}

fn get_kind(s: &str) -> &str {
    match s {
        "const char *" => "string",
        "int32_t" => "i32",
        "int64_t" => "i64",
        "float32_t" => "f32",
        "float64_t" => "f64",
        _ => "i32",
    }
}

fn get_wasm_kind(s: &str) -> &str {
    match s {
        "const char *" => "WASM_STRING",
        "int32_t" => "WASM_I32",
        "int64_t" => "WASM_I64",
        "float32_t" => "WASM_F32",
        "float64_t" => "WASM_F64",
        _ => "WASM_S32",
    }
}

fn check_initialize() -> String {
    let mut out = String::new();
    out.push_str("if (!wasm_instance_init) { printf(\"Module Uninitialized\\n\"); exit(-1); }\n");
    out
}
