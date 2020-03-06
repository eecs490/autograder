use array_macro::array;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ if n > 0 => fib(n - 1) + fib(n - 2),
        _ => panic!("fib only accepts positive numbers."),
    }
}

//#[test]
//#[should_panic]
//fn test_panic() {
//submission::fib(-1);
//}

#[test]
fn test_random1() {
    let seed: [u8; 32] = array![|x| x as u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let n: i32 = rng.gen_range(0, 10);
    assert_eq!(submission::fib(n), fib(n), "input: {}", n);
}

#[test]
fn test_random2() {
    let seed: [u8; 32] = array![|x| (x + 1) as u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let n: i32 = rng.gen_range(0, 10);
    assert!(submission::fib(n) == fib(n), "input: {}", n);
}
