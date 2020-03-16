#![feature(try_trait)]
mod args;
mod cargo_test_output;
mod error;
mod report;
mod score_map;
use args::Args;
use cargo_test_output::TestOutputs;
use clap;
use either::{Left, Right};
use error::{
    AssertionError, FileCreationError, LcovReadError, MyError, ReadError, ReportError,
    TestOutputError, TestReportError, WriteError,
};
use lcov::Reader;
use report::{Report, TestReport};
use score_map::ScoreMap;
use serde_json::to_string_pretty;
use snafu::ensure;
use snafu::IntoError;
use snafu::ResultExt;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter::once;

pub type Result<T, E = MyError> = std::result::Result<T, E>;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {}", e);
    }
}

fn run() -> Result<()> {
    let matches = Args::get();

    let output_path = matches.get_path_buf("output")?;
    let lcov_path = matches.get_path_buf("lcov")?;
    let scores_path = matches.get_path_buf("scores")?;
    let our_test_outputs = matches.get_path_buf("our_test_outputs")?;
    let their_test_outputs = matches.get_path_buf("their_test_outputs")?;

    // coerce to paths
    let output_path = output_path.as_path();
    let lcov_path = lcov_path.as_path();
    let scores_path = scores_path.as_path();
    let our_test_outputs = our_test_outputs.as_path();
    let their_test_outputs = their_test_outputs.as_path();

    // assign custom scores to each test function.
    let scores: ScoreMap = ScoreMap::from_path(scores_path)?;

    // deserialize ouputs into TestOutput structs
    let mut our_test_outputs: TestOutputs = TestOutputs::from_path(our_test_outputs)?;

    let formatted_test_names = our_test_outputs.names().collect::<Vec<_>>().join("\n");
    let our_test_names = our_test_outputs.names().collect::<HashSet<_>>();
    let score_names = scores.our_test_names().collect::<HashSet<_>>();
    let formatted_score_names = scores.our_test_names().collect::<Vec<_>>().join("\n");
    let assertion = our_test_names == score_names;
    let msg = format!("There is a mismatch between the test names in scores.yaml:\n{}\nand the assignment tests that ran and completed on the submission code:\n{:?}", formatted_score_names, formatted_test_names);
    ensure!(assertion, AssertionError { msg });

    let their_test_outputs: TestOutputs = TestOutputs::from_path(their_test_outputs)?;
    let mut their_test_outputs: TestOutputs = their_test_outputs.assign_scores(&scores);

    our_test_outputs.sort_by(|r1, r2| r1.name.cmp(&r2.name));
    their_test_outputs.sort_by(|r1, r2| r1.name.cmp(&r2.name));

    println!("Our TestOutput structs:");
    for output in our_test_outputs.clone() {
        println!(
            "{}",
            to_string_pretty(&output).context(TestOutputError { output })?
        );
    }
    println!("Their TestOutput structs:");
    for output in their_test_outputs.clone() {
        println!(
            "{}",
            to_string_pretty(&output).context(TestOutputError { output })?
        );
    }

    // Read lcov.info file
    let lcov_string = fs::read_to_string(&lcov_path).context(ReadError { path: lcov_path })?;
    let records = Reader::new(lcov_string.as_bytes())
        .collect::<std::result::Result<Vec<_>, lcov::reader::Error>>();
    let records = records.map_err(|_| {
        LcovReadError {
            string: lcov_string.clone(),
        }
        .into_error(snafu::NoneError)
    })?;

    println!("LCov records:");
    for record in records.clone() {
        println!("{:?}", record)
    }
    let coverage_output = format!(
        "\
    Score is based on the following LCOV coverage data output:

    {}

    To create an HTML view of LCOV data:
    - navigate to the root of your submission
    - copy LCOV data to a file `lcov.info`
    - run `mkdir -p /tmp/ccov && genhtml -o /tmp/ccov --show-details --highlight --ignore-errors source --legend lcov.info`", &lcov_string);

    // Covert TestOutputs into TestReports
    let num_their_tests = their_test_outputs.len() as f32;
    let test_reports = our_test_outputs
        .into_test_reports("Our tests".into(), Right(&scores))
        .chain(their_test_outputs.into_test_reports(
            "Your tests".into(),
            Left(scores.their_tests / num_their_tests),
        ))
        // Convert lcov records into TestReports and append to test_reports vec
        .chain(once(TestReport::line_coverage(
            &records,
            scores.line_coverage,
            Some(coverage_output),
        )))
        .collect::<Result<Vec<_>>>()?;

    // Collect the read records into a vector.
    println!("TestReport structs:");
    for report in test_reports.clone() {
        println!(
            "{}",
            to_string_pretty(&report).context(TestReportError { report })?
        );
    }

    // combine TestOutput structs into Report struct
    let report: Report = Report::build(test_reports, &scores, None)?;
    println!("Gradescope Report:");
    println!(
        "{}",
        to_string_pretty(&report.clone()).context(ReportError {
            report: report.clone()
        })?
    );

    // write Report object to output_path
    let mut buffer = File::create(output_path).context(FileCreationError { path: output_path })?;
    buffer
        .write(
            &serde_json::to_string(&report)
                .context(ReportError { report })?
                .as_bytes(),
        )
        .context(WriteError { path: output_path })?;
    Ok(())
}
