use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use test_lib;
use test_lib::Report;
use test_lib::TestResult;

#[test]
#[should_panic]
fn test_panic() {
    submission::fib(-1);
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
    let scores: HashMap<String, f32> = map! { "tests::test4" => 5.0 }; // assign custom scores to each test function. The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let args: Vec<String> = env::args().collect();
    let submission_path = args
        .get(1)
        .expect("Must provide one argument representing path to submission Cargo.toml.");
    let assignment_path = args
        .get(2)
        .expect("Must provide one argument representing path to assignment Cargo.toml.");
    let output_path = args
        .get(3)
        .expect("Must provide one argument representing path to write results file.");

    // scrape cargo test for submission and assignment package
    let outputs: (String, String) = (
        test_lib::get_test_output(assignment_path),
        test_lib::get_test_output(submission_path),
    );
    println!("{}", outputs.0.clone());
    println!("{}", outputs.1.clone());

    // deserialize ouputs into TestResult structs
    let mut test_results: (Vec<TestResult>, Vec<TestResult>) = (
        test_lib::get_test_results(outputs.0),
        test_lib::get_test_results(outputs.1),
    );
    test_results.0.extend(test_results.1);

    // combine TestResult structs into Report struct
    let report: Report = test_lib::build_report(test_results.0, scores);
    println!("{}", report.clone().to_string());

    // write Report object to output_path
    let mut buffer = File::create(output_path)?;
    buffer.write(&report.to_string().as_bytes())?;
    Ok(())
}
