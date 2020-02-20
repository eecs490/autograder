use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

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
    score: String,
    max_score: f32,
    name: String,
    number: String,
    output: Option<String>,
    tags: Option<std::vec::Vec<String>>,
    visibility: Option<Visibility>,
}

impl TestReport {
    pub fn new(
        score: f32,
        max_score: f32,
        name: String,
        number: usize,
        output: Option<String>,
        tags: Option<std::vec::Vec<String>>,
        visibility: Option<Visibility>,
    ) -> TestReport {
        TestReport {
            score: score.to_string(),
            max_score: max_score,
            name: name,
            number: number.to_string(),
            output: output,
            tags: tags,
            visibility: visibility,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    score: String,
    execution_time: Option<f32>,
    output: Option<String>,
    stdout_visibility: Option<Visibility>,
    tests: std::vec::Vec<TestReport>,
}
impl Report {
    pub fn to_string(self) -> String {
        return serde_json::to_string(&self).expect("Failed to produce JSON string.");
    }
    fn new(
        score: f32,
        execution_time: Option<f32>,
        output: Option<String>,
        stdout_visibility: Option<Visibility>,
        tests: std::vec::Vec<TestReport>,
    ) -> Report {
        Report {
            score: score.to_string(),
            execution_time: execution_time,
            output: output,
            stdout_visibility: stdout_visibility,
            tests: tests,
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
    pub fn to_string(self) -> String {
        return serde_json::to_string(&self).expect("Failed to produce JSON string.");
    }
}

fn get_max_score(name: &String, scores: &HashMap<String, f32>) -> f32 {
    *scores.get(name).unwrap_or(&1.0)
}

fn get_score(test_result: &TestResult, scores: &HashMap<String, f32>) -> f32 {
    match test_result.event {
        Event::Ok => get_max_score(&test_result.name, scores),
        Event::Failed => 0.0,
    }
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

pub fn build_report(test_results: Vec<TestResult>, scores: HashMap<String, f32>) -> Report {
    let actual_score: f32 = test_results
        .clone()
        .into_iter()
        .map(|r| get_score(&r, &scores))
        .sum();
    let max_score: f32 = test_results
        .clone()
        .into_iter()
        .map(|r| get_max_score(&r.name, &scores))
        .sum();
    let test_reports: Vec<TestReport> = test_results
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            TestReport::new(
                get_score(&r, &scores),
                get_max_score(&r.name.clone(), &scores),
                r.name,
                i,
                r.stdout.or(r.message),
                None,
                None,
            )
        })
        .collect();
    Report::new(
        100.0 * actual_score / max_score,
        None,
        None,
        None,
        test_reports,
    )
}
