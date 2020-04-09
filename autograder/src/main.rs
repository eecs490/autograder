#![feature(try_trait)]
mod cargo_test_output;
mod error;
mod labels;
mod opt;
mod report;
mod score_map;
use cargo_test_output::TestOutputs;
use either::{Left, Right};
use error::{
    AssertionError, FileCreationError, LcovReadError, MyError, ReadError, ReportError,
    TestOutputError, TestReportError, WriteError,
};
use labels::Labels;
use lcov::Reader;
use opt::Opt;
use report::{line_coverage, Report, TestReport};
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
use structopt::StructOpt;

pub type Result<T, E = MyError> = std::result::Result<T, E>;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {}", e);
    }
}

fn run() -> Result<()> {
    let opt = Opt::from_args();

    // coerce to paths
    let output_path = opt.output.as_path();
    let lcov_path = opt.lcov.as_path();
    let scores_path = opt.scores.as_path();
    let labels_path = opt.labels.as_path();
    let our_test_outputs = opt.our_test_outputs.as_path();
    let their_test_outputs = opt.their_test_outputs.as_path();

    // Covert TestOutputs into TestReports
    let num_their_tests = their_test_outputs.len() as f32;
    let test_reports = our_test_outputs
        .into_test_reports(labels.our_tests, Right(&scores))
        .chain(their_test_outputs.into_test_reports(
            labels.their_tests,
            Left(scores.their_tests / num_their_tests),
        ))
        // Convert lcov records into TestReports and append to test_reports vec
        .chain(once(TestReport::coverage_result(
            line_coverage(&records),
            scores.line_coverage,
            labels.line_coverage,
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
