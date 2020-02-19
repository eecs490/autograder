use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use test_lib;

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
    let scores: HashMap<String, f32> = map! { "tests::test4" => 5.0 };
    let _ = test_lib::Type::Suite;
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
    let submission_output = test_lib::get_test_output(submission_path);
    println!("{}", submission_output);
    let assignment_output = test_lib::get_test_output(assignment_path);
    println!("{}", assignment_output);
    let mut test_results = test_lib::get_test_results(submission_output);
    test_results.extend(test_lib::get_test_results(assignment_output));
    let report = test_lib::build_report(test_results, scores).to_string();
    println!("{}", report);
    let mut buffer = File::create(output_path)?;
    buffer.write(&report.as_bytes())?;
    Ok(())
}
