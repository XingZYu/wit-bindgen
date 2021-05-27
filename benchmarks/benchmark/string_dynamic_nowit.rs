static mut STRING: String = String::new();

pub fn write(string: &str) -> () {
    unsafe {
        STRING = String::from(string);
    }
    ()
}

pub fn read() -> String {
    let ret = unsafe {
        String::from(&STRING)
    };
    ret 
}