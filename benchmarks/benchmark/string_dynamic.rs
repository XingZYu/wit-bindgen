use wasm_bindgen::prelude::*;

static mut STRING: String = String::new();

#[wasm_bindgen]
pub fn write(string: &str) -> () {
    unsafe {
        STRING = String::from(string);
    }
    ()
}

#[wasm_bindgen]
pub fn read() -> String {
    let ret = unsafe {
        String::from(&STRING)
    };
    ret 
}