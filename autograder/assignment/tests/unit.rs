use array_macro::array;
use assignment::solution::fib;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

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
    assert_eq!(submission::solution::fib(n), fib(n), "input: {}", n);
}

#[test]
fn test_random2() {
    let seed: [u8; 32] = array![|x| (x + 1) as u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let n: i32 = rng.gen_range(0, 10);
    assert!(submission::solution::fib(n) == fib(n), "input: {}", n);
}
