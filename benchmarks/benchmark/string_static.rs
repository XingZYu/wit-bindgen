use wasm_bindgen::prelude::*;

static STRING: &str = "Hello WebAssembly: This is a static string";

#[wasm_bindgen]
pub fn read() -> String {
    String::from(STRING)
}
