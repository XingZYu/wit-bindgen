static STRING: &str = "Hello WebAssembly: This is a static string";

pub fn read() -> String {
    String::from(STRING)
}
