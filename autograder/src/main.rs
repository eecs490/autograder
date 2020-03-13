// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;
//mod error;
mod report;
mod score_map;
mod test_result;
use clap::{value_t, App, Arg};
//use error::Error;
use lcov::reader::Error::{Io, ParseRecord};
use lcov::Reader;
use report::{Report, TestReport};
use score_map::ScoreMap;
use serde_json::to_string_pretty;
use std::collections::HashSet;
use std::fs;
//use std::fs::File;
//use std::io::Write;
//use std::iter::once;
use clap;
use std::path::PathBuf;
use test_result::TestResult;
mod error {
    error_chain! {}
}

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Clap(::clap::Error);
        Yaml(::serde_yaml::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
    }
    errors {
        ScoreError(s: String) {
            display("Name not found in scores.yaml file: '{}'", s)
        }
        LcovReaderError(e: lcov::reader::Error) {
            display("Unable to read {}", e)
        }
    }
}

impl From<lcov::reader::Error> for Error {
    fn from(err: lcov::reader::Error) -> Error {
        Error::from(ErrorKind::LcovReaderError(err))
        //Error::from(match err {
        //Io(e) => ErrorKind::Io(e),
        //ParseRecord(_, _) => panic!("oh shit"), // ErrorKind::LcovReaderError(err),
        //})
    }
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
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

    //// coerce to paths
    let _output_path = output_path.as_path();
    let _lcov_path = lcov_path.as_path();
    let scores_path = scores_path.as_path();
    let our_test_results = our_test_results.as_path();
    let their_test_results = their_test_results.as_path();

    // assign custom scores to each test function.
    // The autograder defaults to 1.0 point per test for tests not included in thei HashMap.
    let scores: ScoreMap = ScoreMap::from_path(scores_path)?;

    // deserialize ouputs into TestResult structs
    let mut our_test_results: Vec<TestResult> = TestResult::from_path(our_test_results)?;

    assert_eq!(
    scores.our_test_names().collect::<HashSet<String>>(),
    our_test_results
    .iter()
    .map(|r| r.name.clone())
    .collect::<HashSet<String>>(),
    "There is a mismatch between the test names in scores.yaml and the assignment tests that ran and completed on the submission code."
    );

    let their_test_results: Vec<TestResult> = TestResult::from_path(their_test_results)?;
    let mut their_test_results: Vec<TestResult> = their_test_results
        .iter()
        .map(|r| r.assign_score(scores.their_tests))
        .collect();

    our_test_results.sort_by(|r1, r2| r1.name.cmp(&r2.name));
    their_test_results.sort_by(|r1, r2| r1.name.cmp(&r2.name));

    println!("Our TestResult structs:");
    for result in our_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }
    println!("Their TestResult structs:");
    for result in their_test_results.clone() {
        println!("{}", to_string_pretty(&result)?);
    }

    // Read lcov.info file
    let lcov_string = fs::read_to_string(lcov_path)?;
    let reader = Reader::new(lcov_string.as_bytes());
    let records = reader.collect::<std::result::Result<Vec<_>, _>>()?;

    println!("LCov records:");
    for record in records.clone() {
        println!("{:?}", record)
    }
    let coverage_output = Some(format!(
    "Score is based on the following LCOV coverage data output:

    {}

    To create an HTML view of LCOV data:
    - navigate to the root of your submission
    - copy LCOV data to a file `lcov.info`
    - run `mkdir -p /tmp/ccov && genhtml -o /tmp/ccov --show-details --highlight --ignore-errors source --legend lcov.info`", lcov_string));

    // Covert TestResults into TestReports
    let num_their_tests = their_test_results.len() as f32;
    //let test_reports = our_test_results
    //.iter()
    //.map(|r| TestReport::from_our_tests(r, "Our tests".into(), &scores))
    //.chain(their_test_results.iter().map(|r| {
    //TestReport::from_their_tests(
    //&r,
    //"Your tests".into(),
    //scores.their_tests / num_their_tests,
    //)
    //}))
    //// Convert lcov records into TestReports and append to test_reports vec
    //.chain(once(TestReport::line_coverage(
    //&records,
    //"".into(),
    //scores.line_coverage,
    //coverage_output.clone(),
    //)))
    //.collect::<Result<Vec<_>, _>>()?;

    //// Collect the read records into a vector.
    //println!("TestReport structs:");
    //for report in test_reports.clone() {
    //println!("{}", to_string_pretty(&report)?);
    //}

    //// combine TestResult structs into Report struct
    //let report: Report = Report::build(test_reports, &scores, None)?;
    //println!("Gradescope Report:");
    //println!("{}", to_string_pretty(&report)?);

    //// write Report object to output_path
    //let mut buffer = File::create(output_path).map_err(|e| Error::io_error_from(e, output_path))?;
    //buffer
    //.write(&serde_json::to_string(&report)?.as_bytes())
    //.map_err(|e| Error::io_error_from(e, output_path))?;
    Ok(())
}
