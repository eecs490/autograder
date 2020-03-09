#![feature(try_trait)]
mod error;
mod report;
use lcov::Reader;
use report::records_to_string;
mod test_result;
mod util;
extern crate array_macro;
extern crate rand;
use clap::{App, Arg};
use error::Error;
use report::{GradescopeReport, Report, TestReport};
use serde_json::to_string_pretty;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::Write;
use test_result::TestResult;

fn main() -> Result<(), Error> {
    let matches = App::new("MyApp")
        .arg(
            Arg::with_name("our_test_results")
                .long("our-test-results")
                .help("path to output of running our tests on their solution")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("their_test_results")
                .long("their-test-results")
                .help("path to output of running their tests on our solution")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("submission")
                .long("submission")
                .help("path to submission/Cargo.toml")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .help("path where results.json will be written")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("lcov")
                .long("lcov")
                .help("path to lcov.info")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("scores")
                .help("path to scores.yaml")
                .long("scores")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("our_solution")
                .long("our-solution")
                .help("path to our solution.rs file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("their_solution")
                .long("their-solution")
                .help("path to their solution.rs file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let output_path = matches.value_of("output").unwrap();
    let lcov_path = matches.value_of("lcov").unwrap();
    let scores_path = matches.value_of("scores").unwrap();
    let _our_solution = matches.value_of("our_solution").unwrap();
    let _their_solution = matches.value_of("their_solution").unwrap();
    let our_test_results = matches.value_of("our_test_results").unwrap();
    let _their_test_results = matches.value_of("their_test_results").unwrap();

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores = fs::read_to_string(scores_path)?;
    let scores: util::ScoreMap = serde_yaml::from_str(&scores)?;

    // deserialize ouputs into TestResult structs
    let our_test_results = fs::read_to_string(our_test_results)?;
    let test_results: Vec<TestResult> = TestResult::from_output(our_test_results);
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
    test_reports.push(TestReport::line_coverage(
        &records,
        test_reports.len(),
        &scores,
    ));
    test_reports.push(TestReport::branch_coverage(
        &records,
        test_reports.len(),
        &scores,
    ));

    // Collect the read records into a vector.
    println!("TestReport structs:");
    for report in test_reports.clone() {
        println!("{}", to_string_pretty(&report)?);
    }

    // combine TestResult structs into Report struct
    let output = format!(
"Coverage scores are based on the following <code>lcov</code> coverage data output:
\n{}\n\n
To create an HTML view of this data, navigate to the root of your submission, create a file `lcov.info`, and run `mkdir -p /tmp/ccov && genhtml -o /tmp/ccov --show-details --highlight --ignore-errors source --legend lcov.info`.",
records_to_string(&records)
);
    let report: Report = Report::build(test_reports, &scores, Some(output));
    let gradescope_report: GradescopeReport = GradescopeReport::from(report);
    println!("Gradescope Report:");
    println!("{}", to_string_pretty(&gradescope_report)?);

    // write Report object to output_path
    let mut buffer = File::create(output_path.to_string())?;
    buffer.write(&serde_json::to_string(&gradescope_report)?.as_bytes())?;
    Ok(())
}
