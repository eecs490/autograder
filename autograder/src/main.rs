mod error;
mod report;
mod score_map;
mod test_result;
use clap::{value_t, App, Arg};
use error::Error;
use lcov::Reader;
use report::records_to_string;
use report::{GradescopeReport, Report, TestReport};
use score_map::ScoreMap;
use serde_json::to_string_pretty;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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
        .get_matches();

    // parse args
    let output_path = value_t!(matches, "output", PathBuf)?;
    let lcov_path = value_t!(matches, "lcov", PathBuf)?;
    let scores_path = value_t!(matches, "scores", PathBuf)?;
    let our_test_results = value_t!(matches, "our_test_results", PathBuf)?;
    let their_test_results = value_t!(matches, "their_test_results", PathBuf)?;

    // coerce to paths
    let output_path = output_path.as_path();
    let lcov_path = lcov_path.as_path();
    let scores_path = scores_path.as_path();
    let our_test_results = our_test_results.as_path();
    let their_test_results = their_test_results.as_path();

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: ScoreMap = ScoreMap::from_path(scores_path)?;

    // deserialize ouputs into TestResult structs
    let our_test_results: Vec<TestResult> = TestResult::from_path(our_test_results)?;
    let their_test_results: Vec<TestResult> = TestResult::from_path(their_test_results)?;
    let their_test_results = their_test_results
        .iter()
        .map(|r| r.assign_score(scores.their_tests));

    println!("Our TestResult structs:");
    for result in our_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }
    println!("Their TestResult structs:");
    for result in their_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }

    // Covert TestResults into TestReports
    let num_their_tests = their_test_results.len() as f32;
    let mut test_reports = our_test_results
        .iter()
        .enumerate()
        .map(|(i, r)| TestReport::from_our_tests(r, i, &scores))
        .chain(their_test_results.enumerate().map(|(i, r)| {
            TestReport::from_their_tests(&r, i, scores.their_tests / num_their_tests)
        }))
        .collect::<Result<Vec<_>, _>>()?;

    // Read lcov.info file
    let readers = Reader::open_file(lcov_path).map_err(|e| Error::io_error_from(e, lcov_path))?;
    let records = readers.collect::<Result<Vec<_>, _>>()?;
    println!("LCov records:");
    for record in records.clone() {
        println!("{:?}", record)
    }

    // Convert lcov records into TestReports and append to test_reports vec
    let coverage_output = Some(format!(
        "Score is based on the following LCOV coverage data output:
    \n{}",
        records_to_string(&records)
    ));
    test_reports.push(TestReport::line_coverage(
        &records,
        test_reports.len(),
        scores.line_coverage,
        coverage_output.clone(),
    )?);
    test_reports.push(TestReport::branch_coverage(
        &records,
        test_reports.len(),
        scores.branch_coverage,
        coverage_output.clone(),
    )?);

    // Collect the read records into a vector.
    println!("TestReport structs:");
    for report in test_reports.clone() {
        println!("{}", to_string_pretty(&report)?);
    }

    // combine TestResult structs into Report struct
    let output = Some("To create an HTML view of LCOV data:\n- navigate to the root of your submission\n- copy LCOV data to a file `lcov.info`\n- run `mkdir -p /tmp/ccov && genhtml -o /tmp/ccov --show-details --highlight --ignore-errors source --legend lcov.info`".into());
    let report: Report = Report::build(test_reports, &scores, output)?;
    let gradescope_report: GradescopeReport = GradescopeReport::from(report.clone());
    println!("Gradescope Report:");
    println!("{}", to_string_pretty(&gradescope_report)?);

    assert_eq!(
        report.names().collect::<HashSet<String>>(),
        scores.our_test_names().collect::<HashSet<String>>()
    );

    //for (n1, n2) in scores.our_test_names().zip_longest(scores.names()) {}

    // write Report object to output_path
    let mut buffer = File::create(output_path).map_err(|e| Error::io_error_from(e, output_path))?;
    buffer
        .write(&serde_json::to_string(&gradescope_report)?.as_bytes())
        .map_err(|e| Error::io_error_from(e, output_path))?;
    Ok(())
}
