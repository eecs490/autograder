mod lib;
extern crate array_macro;
extern crate rand;
use lib::Report;
use lib::TestResult;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

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

    let outputs: (String, String) = (
        lib::get_test_output(assignment_path.to_string()),
        lib::get_test_output(submission_path.to_string()),
    );
    println!("{}", outputs.0.clone());
    println!("{}", outputs.1.clone());

    // deserialize ouputs into TestResult structs
    let mut test_results: (Vec<TestResult>, Vec<TestResult>) = (
        lib::get_test_results(outputs.0),
        lib::get_test_results(outputs.1),
    );
    test_results.0.extend(test_results.1.clone());
    test_results
        .0
        .clone()
        .into_iter()
        .for_each(|r| println!("{}", r.to_string()));

    // combine TestResult structs into Report struct
    let report: Report = lib::build_report(test_results.0, scores);
    println!("{}", report.clone().to_string());

    // write Report object to output_path
    let mut buffer = File::create(output_path.to_string())?;
    buffer.write(&report.to_string().as_bytes())?;
    Ok(())
}
