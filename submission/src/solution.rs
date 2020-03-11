pub fn fib(n: i32) -> i32 {
    match n {
        0 => fib(0),
        1 => 1,
        _ if n > 0 => fib(n - 1) + fib(n - 2),
        _ => panic!("fib only accepts positive numbers."),
    }
}
