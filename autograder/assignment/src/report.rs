use crate::error::Error;
use crate::test_result::TestResult;
use crate::util::get_max_score;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tarpaulin::config::types::OutputFile;
use tarpaulin::config::Config;
use tarpaulin::report::json::CoverageReport;
use tarpaulin::trace;
use tarpaulin::traces::TraceMap;

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

impl Report {
    pub fn build(test_reports: Vec<TestReport>, scores: &HashMap<String, f32>) -> Self {
        let actual_score: f32 = test_reports.clone().into_iter().map(|r| r.score).sum();
        let max_score: f32 = test_reports
            .clone()
            .into_iter()
            .map(|r| get_max_score(&r.name, scores))
            .sum();
        Self {
            score: 100.0 * actual_score / max_score,
            execution_time: None,
            output: None,
            stdout_visibility: None,
            tests: test_reports,
        }
    }
}

impl TestReport {
    pub fn from_result(result: &TestResult, number: usize, scores: &HashMap<String, f32>) -> Self {
        Self {
            score: result.get_score(scores),
            max_score: get_max_score(&result.name.clone(), scores),
            name: result.name.clone(),
            number: number,
            output: result.stdout.clone().or(result.message.clone()),
            tags: None,
            visibility: None,
        }
    }

    pub fn coverage(submission_path: String, max_score: f32) -> Result<TestReport, Error> {
        let mut config = Config::default();
        config.manifest = PathBuf::from(submission_path);
        config.generate = vec![OutputFile::Json];
        //config.output_directory = PathBuf::from("/tmp");
        let tracemap: Result<TraceMap, Error> = trace(&[config]).map_err(Error::from);
        let coverage_report = CoverageReport::from(&tracemap?);
        let covered: usize = coverage_report.covered().iter().sum();
        let coverable: usize = coverage_report.coverable().iter().sum();
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
}
