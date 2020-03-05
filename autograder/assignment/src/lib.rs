use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tarpaulin::config::types::OutputFile;
use tarpaulin::config::Config;
use tarpaulin::errors::RunError;
use tarpaulin::report::json::CoverageReport;
use tarpaulin::trace;
use tarpaulin::traces::TraceMap;

#[macro_export]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Hidden,
    AfterDueDate,
    AfterPublished,
    Visible, // default
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TestReport {
    score: f32,
    max_score: f32,
    name: String,
    number: usize,
    output: Option<String>,
    tags: Option<std::vec::Vec<String>>,
    visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GradescopeTestReport {
    score: String,
    max_score: f32,
    name: String,
    number: String,
    output: Option<String>,
    tags: Option<std::vec::Vec<String>>,
    visibility: Option<Visibility>,
}

impl From<TestReport> for GradescopeTestReport {
    fn from(report: TestReport) -> Self {
        GradescopeTestReport {
            score: report.score.to_string(),
            max_score: report.max_score,
            name: report.name,
            number: report.number.to_string(),
            output: report.output,
            tags: report.tags,
            visibility: report.visibility,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    score: f32,
    execution_time: Option<f32>,
    output: Option<String>,
    stdout_visibility: Option<Visibility>,
    tests: std::vec::Vec<TestReport>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GradescopeReport {
    score: String,
    execution_time: Option<f32>,
    output: Option<String>,
    stdout_visibility: Option<Visibility>,
    tests: std::vec::Vec<GradescopeTestReport>,
}

impl GradescopeReport {
    pub fn to_string(self) -> String {
        return serde_json::to_string(&self).expect("Failed to produce JSON string.");
    }
}

impl From<Report> for GradescopeReport {
    fn from(report: Report) -> Self {
        GradescopeReport {
            score: report.score.to_string(),
            execution_time: report.execution_time,
            output: report.output,
            stdout_visibility: report.stdout_visibility,
            tests: report
                .tests
                .into_iter()
                .map(GradescopeTestReport::from)
                .collect(),
        }
    }
}

//{ "type": "suite", "event": "started", "test_count": 5 }
//{ "type": "test", "event": "started", "name": "tests::test0" }
//{ "type": "test", "event": "started", "name": "tests::test1" }
//{ "type": "test", "event": "started", "name": "tests::test2" }
//{ "type": "test", "event": "started", "name": "tests::test3" }
//{ "type": "test", "event": "started", "name": "tests::test4" }
//{ "type": "test", "name": "tests::test0", "event": "ok" }
//{ "type": "test", "name": "tests::test1", "event": "ok" }
//{ "type": "test", "name": "tests::test2", "event": "ok" }
//{ "type": "test", "name": "tests::test3", "event": "ok" }
//{ "type": "test", "name": "tests::test4", "event": "failed", "stdout": "thread 'tests::test4' panicked at 'assertion failed: `(left == right)`\n  left: `4`,\n right: `5`: NOOOOOO', src/lib.rs:27:9\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
//{ "type": "suite", "event": "failed", "passed": 4, "failed": 1, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0 }

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Suite,
    Test,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Ok,
    Failed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TestResult {
    #[serde(alias = "type")]
    _type: Type,
    name: String,
    event: Event,
    stdout: Option<String>,
    message: Option<String>,
}

impl TestResult {
    fn get_score(&self, scores: &HashMap<String, f32>) -> f32 {
        match self.event {
            Event::Ok => get_max_score(&self.name, scores),
            Event::Failed => 0.0,
        }
    }
}

fn get_max_score(name: &String, scores: &HashMap<String, f32>) -> f32 {
    *scores.get(name).unwrap_or(&1.0)
}

pub fn get_test_output(path: String) -> String {
    //cargo test --manifest-path="../../Cargo.toml"  -- -Z unstable-options --format json -q
    let stdout = Command::new("cargo")
        .arg("test")
        .arg(format!("--manifest-path={}", path))
        .arg("--")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to capture output")
        .stdout;
    String::from_utf8(stdout).expect("Failed to convert stdout to string.")
}

pub fn get_test_results(test_output: String) -> Vec<TestResult> {
    test_output
        .split("\n")
        .map(serde_json::from_str)
        .filter_map(Result::ok)
        .collect()
}

pub fn get_coverage_result(
    submission_path: String,
    max_score: f32,
) -> Result<TestReport, std::io::Error> {
    let mut config = Config::default();
    config.manifest = PathBuf::from(submission_path);
    config.generate = vec![OutputFile::Json];
    //config.output_directory = PathBuf::from("/tmp");
    let tracemap: Result<TraceMap, std::io::Error> = trace(&[config]).map_err(RunError::into);
    let coverage_report = CoverageReport::from(&tracemap?);
    let covered: usize = coverage_report.iter().map(|f| f.covered).sum();
    let coverable: usize = coverage_report.iter().map(|f| f.coverable).sum();
    Ok(TestReport {
        score: covered as f32 / coverable as f32,
        max_score: max_score,
        name: String::from("test coverage"),
        number: 0,
        output: None,
        tags: None,
        visibility: None,
    })
}

pub fn test_report_from_result(
    result: &TestResult,
    number: usize,
    scores: &HashMap<String, f32>,
) -> TestReport {
    TestReport {
        score: result.get_score(scores),
        max_score: get_max_score(&result.name.clone(), scores),
        name: result.name.clone(),
        number: number,
        output: result.stdout.clone().or(result.message.clone()),
        tags: None,
        visibility: None,
    }
}

pub fn build_report(test_reports: Vec<TestReport>, scores: &HashMap<String, f32>) -> Report {
    let actual_score: f32 = test_reports.clone().into_iter().map(|r| r.score).sum();
    let max_score: f32 = test_reports
        .clone()
        .into_iter()
        .map(|r| get_max_score(&r.name, scores))
        .sum();
    Report {
        score: 100.0 * actual_score / max_score,
        execution_time: None,
        output: None,
        stdout_visibility: None,
        tests: test_reports,
    }
}
