extern crate array_macro;
extern crate rand;
use std::collections::HashMap;
use std::env;
use test_lib::map;

#[cfg(test)]
mod tests {
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

    #[test]
    #[should_panic]
    fn test_panic() {
        submission::fib(-1);
    }

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
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let submission_path = args
        .get(1)
        .expect("Must provide one argument representing path to submission/Cargo.toml.");
    let assignment_path = args
        .get(2)
        .expect("Must provide one argument representing path to assignment/Cargo.toml.");
    let output_path = args
        .get(3)
        .expect("Must provide one argument representing path to write results file.");

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: HashMap<String, f32> = map! { "tests::test4" => 5.0 };

    test_lib::write_report(scores, submission_path, assignment_path, output_path)
}
