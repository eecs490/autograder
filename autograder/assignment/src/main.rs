mod error;
mod report;
use lcov::{Reader, Record, RecordKind};
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
    // `Reader` is an iterator that iterates over `Result<lcov::Record, E>` read from the input buffer.
    let reader = Reader::open_file("/Users/ethan/autograder/autograder/ccov/lcov.info")?;

    // Collect the read records into a vector.
    let records = reader.collect::<Result<Vec<_>, _>>()?;
    assert_eq!(records[0], Record::TestName { name: "".into() });
    assert_eq!(records[1].kind(), RecordKind::SourceFile);

    // Outputs the read records in LCOV tracefile format.
    for record in records {
        println!("{}", record);
    }
    Ok(())
}

fn main2() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let submission_path = args.get(1).ok_or(Error::arg(
        "Must provide one argument representing path to submission/Cargo.toml.",
    ))?;
    let assignment_path = args.get(2).ok_or(Error::arg(
        "Must provide one argument representing path to assignment/Cargo.toml.",
    ))?;
    let output_path = args.get(3).ok_or(Error::arg(
        "Must provide one argument representing path to write results file.",
    ))?;

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: HashMap<String, f32> = map! { "tests::test4" => 5.0 };

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
    let mut test_reports: Vec<TestReport> = test_results
        .iter()
        .enumerate()
        .map(|(i, r)| TestReport::from_result(r, i + 1, &scores))
        .collect();

    // `Reader` is an iterator that iterates over `Result<lcov::Record, E>` read from the input buffer.
    let mut reader = Reader::open_file("tests/fixtures/report.info")?;

    // Collect the read records into a vector.
    let records = reader.collect::<Result<Vec<_>, _>>()?;
    assert_eq!(records[0], Record::TestName { name: "".into() });
    assert_eq!(records[1].kind(), RecordKind::SourceFile);

    // Outputs the read records in LCOV tracefile format.
    for record in records {
        println!("{}", record);
    }
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
