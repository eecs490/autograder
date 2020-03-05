mod lib;
extern crate array_macro;
extern crate rand;
use lib::Report;
use lib::TestReport;
use lib::TestResult;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

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

    // scrape cargo test output for assignment and submission
    let outputs: String = lib::get_test_output(assignment_path.to_string());
    println!("{}", outputs.clone());

    // deserialize ouputs into TestResult structs
    let test_results: Vec<TestResult> = lib::get_test_results(outputs);
    test_results
        .clone()
        .into_iter()
        .for_each(|r| println!("{}", r.to_string()));
    let mut test_reports: Vec<TestReport> = test_results
        .iter()
        .enumerate()
        .map(|(i, r)| lib::test_report_from_result(r, i, &scores))
        .collect();

    let coverage_result = lib::get_coverage_result(submission_path.to_string(), 10.0);

    test_reports.push(coverage_result?);

    // combine TestResult structs into Report struct
    let report: Report = lib::build_report(test_reports, &scores);
    println!("{}", report.clone().to_string());

    // write Report object to output_path
    let mut buffer = File::create(output_path.to_string())?;
    buffer.write(&report.to_string().as_bytes())?;
    Ok(())
}
