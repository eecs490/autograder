use crate::test_result::TestResult;
use crate::util::{get_max_score, ScoreMap};
use lcov::Record;
use serde::{Deserialize, Serialize};

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
    pub fn build(test_reports: Vec<TestReport>, scores: &ScoreMap, output: Option<String>) -> Self {
        let actual_score: f32 = test_reports.clone().into_iter().map(|r| r.score).sum();
        let max_score: f32 = test_reports
            .clone()
            .into_iter()
            .map(|r| get_max_score(&r.name, scores))
            .sum();
        Self {
            score: 100.0 * actual_score / max_score,
            execution_time: None,
            output: output,
            stdout_visibility: None,
            tests: test_reports,
        }
    }
}

pub fn line_coverage(records: &Vec<Record>) -> f32 {
    let (lines_hit, lines_found): (u32, u32) =
        records.iter().fold((0, 0), |(h, f), record| match record {
            Record::LinesFound { found } => (h, found + f),
            Record::LinesHit { hit } => (hit + h, f),
            _ => (h, f),
        });
    lines_hit as f32 / lines_found as f32
}

pub fn branch_coverage(records: &Vec<Record>) -> f32 {
    let (branches_hit, branches_found): (u32, u32) =
        records
            .iter()
            .fold((0, 0), |(hit, found), record| match record {
                Record::BranchData { taken: Some(n), .. } if *n > 0 => (hit + 1, found + 1),
                Record::BranchData { .. } => (hit, found + 1),
                _ => (hit, found),
            });
    branches_hit as f32 / branches_found as f32
}

pub fn records_to_string(records: &Vec<Record>) -> String {
    records
        .into_iter()
        .map(|rec| format!("{}\n", rec))
        .collect::<String>()
}

impl TestReport {
    pub fn from_result(result: &TestResult, number: usize, scores: &ScoreMap) -> Self {
        Self {
            score: result.get_score(scores),
            max_score: get_max_score(&result.name.clone(), scores),
            name: result.name.clone(),
            number: number,
            output: result.stdout.clone(),
            tags: None,
            visibility: None,
        }
    }
    pub fn line_coverage(records: &Vec<Record>, number: usize, scores: &ScoreMap) -> Self {
        let name: String = "Line coverage".into();
        let score = get_max_score(&name, scores);
        Self {
            score: score * line_coverage(records),
            max_score: score,
            name: "Line coverage".into(),
            number: number,
            output: None,
            tags: None,
            visibility: None,
        }
    }
    pub fn branch_coverage(records: &Vec<Record>, number: usize, scores: &ScoreMap) -> Self {
        let name: String = "Branch coverage".into();
        let score = get_max_score(&name, scores);
        Self {
            score: score * branch_coverage(records),
            max_score: score,
            name: name,
            number: number,
            output: None,
            tags: None,
            visibility: None,
        }
    }
}
