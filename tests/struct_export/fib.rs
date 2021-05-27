use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Fib {
    fib_high: i32,
}

#[wasm_bindgen]
impl Fib {
    pub fn new() -> Self {
        Fib{ fib_high : 0 }
    }

    pub fn fibonacci(&mut self, n: i32) -> i32 {
        let mut a = 1u64;
        let mut b = 1;
        for _ in 0..n {
            let tmp = b;
            b += a;
            a = tmp;
        }
        unsafe {
            self.fib_high = (a >> 32) as i32;
        }
        return a as i32;
    }
}
