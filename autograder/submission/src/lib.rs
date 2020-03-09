//pub fn _fib(n: i32) -> i32 {
//match n {
//0 => 1,
//_ => n,
//}
//}

pub fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
