use crate::args::args;
use crate::cargo_test_output::TestOutput;
use crate::error::Result;
use crate::error::ResultExt;
use crate::error::{failed_to_read, Error};
use crate::report::{Report, TestReport};
use crate::score_map::ScoreMap;
use clap::value_t;
use lcov::Reader;
use serde_json::to_string_pretty;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::once;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    let matches = args().get_matches();

    // parse args
    let output_path = value_t!(matches, "output", PathBuf)?;
    let lcov_path = value_t!(matches, "lcov", PathBuf)?;
    let scores_path = value_t!(matches, "scores", PathBuf)?;
    let our_test_results = value_t!(matches, "our_test_results", PathBuf)?;
    let their_test_results = value_t!(matches, "their_test_results", PathBuf)?;

    //// coerce to paths
    let _output_path = output_path.as_path();
    let _lcov_path = lcov_path.as_path();
    let scores_path = scores_path.as_path();
    let our_test_results = our_test_results.as_path();
    let their_test_results = their_test_results.as_path();

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: ScoreMap = ScoreMap::from_path(scores_path)?;

    // deserialize ouputs into TestOutput structs
    let mut our_test_results: Vec<TestOutput> = TestOutput::from_path(our_test_results)?;

    assert_eq!(
    scores.our_test_names().collect::<HashSet<String>>(),
    our_test_results
    .iter()
    .map(|r| r.name.clone())
    .collect::<HashSet<String>>(),
    "There is a mismatch between the test names in scores.yaml and the assignment tests that ran and completed on the submission code."
    );

    let their_test_results: Vec<TestOutput> = TestOutput::from_path(their_test_results)?;
    let mut their_test_results: Vec<TestOutput> = their_test_results
        .iter()
        .map(|r| r.assign_score(scores.their_tests))
        .collect();

    our_test_results.sort_by(|r1, r2| r1.name.cmp(&r2.name));
    their_test_results.sort_by(|r1, r2| r1.name.cmp(&r2.name));

    println!("Our TestOutput structs:");
    for result in our_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }
    println!("Their TestOutput structs:");
    for result in their_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }

    // Read lcov.info file
    let lcov_string = fs::read_to_string(&lcov_path).chain_err(|| failed_to_read(&lcov_path))?;
    let reader = Reader::new(lcov_string.as_bytes());
    let records = reader
        .collect::<std::result::Result<Vec<_>, lcov::reader::Error>>()
        .map_err(Error::from)
        .chain_err(|| {
            format!(
                "Unable to parse lcov string: 

{}",
                lcov_string
            )
        })?;

    println!("LCov records:");
    for record in records.clone() {
        println!("{:?}", record)
    }
    let coverage_output = Some(format!(
    "\
Score is based on the following LCOV coverage data output:

{}

To create an HTML view of LCOV data:
- navigate to the root of your submission
- copy LCOV data to a file `lcov.info`
- run `mkdir -p /tmp/ccov && genhtml -o /tmp/ccov --show-details --highlight --ignore-errors source --legend lcov.info`", lcov_string));

    // Covert TestOutputs into TestReports
    let num_their_tests = their_test_results.len() as f32;
    let test_reports = our_test_results
        .iter()
        .map(|r| TestReport::from_our_tests(r, "Our tests".into(), &scores))
        .chain(their_test_results.iter().map(|r| {
            TestReport::from_their_tests(
                &r,
                "Your tests".into(),
                scores.their_tests / num_their_tests,
            )
        }))
        // Convert lcov records into TestReports and append to test_reports vec
        .chain(once(TestReport::line_coverage(
            &records,
            "".into(),
            scores.line_coverage,
            coverage_output.clone(),
        )))
        .collect::<Result<Vec<_>>>()?;

    // Collect the read records into a vector.
    println!("TestReport structs:");
    for report in test_reports.clone() {
        println!("{}", to_string_pretty(&report)?);
    }

    // combine TestOutput structs into Report struct
    let report: Report = Report::build(test_reports, &scores, None)?;
    println!("Gradescope Report:");
    println!("{}", to_string_pretty(&report)?);

    // write Report object to output_path
    let mut buffer = File::create(output_path)?;
    buffer.write(&serde_json::to_string(&report)?.as_bytes())?;
    Ok(())
}
