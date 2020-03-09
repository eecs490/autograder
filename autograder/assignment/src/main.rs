#![feature(try_trait)]
mod error;
mod report;
use lcov::Reader;
mod test_result;
mod util;
extern crate array_macro;
extern crate rand;
use error::Error;
use report::{GradescopeReport, Report, TestReport};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Output;
use test_result::TestResult;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let assignment_path = args.get(1).ok_or(Error::arg(
        "Must provide one argument representing path to assignment/Cargo.toml.",
    ))?;
    let output_path = args.get(2).ok_or(Error::arg(
        "Must provide one argument representing path to write results file.",
    ))?;
    let lcov_path = args.get(3).ok_or(Error::arg(
        "Must provide one argument representing path to lcov.info file.",
    ))?;

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: HashMap<String, f32> = map! { "test_random1" => 5.0 };

    // scrape cargo test output for assignment and submission
    let output: Output = util::cargo_test(assignment_path.to_string())?;
    let stdout = String::from_utf8(output.stdout)?;
    println!("cargo test output:");
    println!("{}", stdout);

    // deserialize ouputs into TestResult structs
    let test_results: Vec<TestResult> = TestResult::from_output(stdout);
    println!("TestResult structs:");
    for result in test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }

    // Covert TestResults into TestReports
    let mut test_reports: Vec<TestReport> = test_results
        .iter()
        .enumerate()
        .map(|(i, r)| TestReport::from_result(r, i, &scores))
        .collect();

    // Read lcov.info file
    let records = Reader::open_file(lcov_path)?.collect::<Result<Vec<_>, _>>()?;
    println!("LCov records:");
    for record in records.clone() {
        println!("{:?}", record)
    }

    // Convert lcov records into TestReports and append to test_reports vec
    test_reports.push(TestReport::line_coverage(&records, test_reports.len(), 2.0));
    test_reports.push(TestReport::branch_coverage(
        &records,
        test_reports.len(),
        2.0,
    ));

    // Collect the read records into a vector.
    println!("TestReport structs:");
    for report in test_reports.clone() {
        println!("{}", to_string_pretty(&report)?);
    }

    // combine TestResult structs into Report struct
    let report: Report = Report::build(test_reports, &scores);
    let gradescope_report: GradescopeReport = GradescopeReport::from(report);
    println!("Gradescope Report:");
    println!("{}", to_string_pretty(&gradescope_report)?);

    // write Report object to output_path
    let mut buffer = File::create(output_path.to_string())?;
    buffer.write(&serde_json::to_string(&gradescope_report)?.as_bytes())?;
    Ok(())
}
