extern crate rand;
use std::collections::HashMap;
use std::env;

#[cfg(test)]
mod tests {
    //use rand::rngs::StdRng;
    //use rand::{Rng, SeedableRng};

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
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key.to_string(), $value);
            )+
            m
        }
     };
);
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
