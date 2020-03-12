pub fn fib(n: i32) -> i32 {
    if n < 0 {
        panic!("oh no")
    }
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
