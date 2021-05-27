pub fn fibonacci(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 0..n {
        let tmp = b;
        b += a;
        a = tmp;
    }
    return a as i32;
}
