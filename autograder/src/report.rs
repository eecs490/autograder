use crate::cargo_test_output::TestOutput;
use crate::score_map::ScoreMap;
use crate::Result;
use lcov::Record;
use serde::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Hidden,
    AfterDueDate,
    AfterPublished,
    Visible, // default
}

fn to_str<S, T>(float: &T, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    s.serialize_str(&*float.to_string())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestReport {
    #[serde(serialize_with = "to_str")]
    score: f32,
    max_score: f32,
    pub name: String,
    #[serde(rename = "number")]
    label: Option<String>,
    output: Option<String>,
    tags: Option<std::vec::Vec<String>>,
    visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Report {
    #[serde(serialize_with = "to_str")]
    score: f32,
    execution_time: Option<f32>,
    output: Option<String>,
    stdout_visibility: Option<Visibility>,
    tests: std::vec::Vec<TestReport>,
}

impl Report {
    pub fn build(
        test_reports: Vec<TestReport>,
        scores: &ScoreMap,
        output: Option<String>,
    ) -> Result<Self> {
        let actual_score: f32 = test_reports.clone().into_iter().map(|r| r.score).sum();
        let max_score: f32 = scores.values().into_iter().sum();

        Ok(Self {
            score: 100.0 * actual_score / max_score,
            execution_time: None,
            output: output,
            stdout_visibility: None,
            tests: test_reports,
        })
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

#[allow(dead_code)]
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

impl TestReport {
    pub fn from_tests(passing: bool, name: String, output: String) -> Result<Self> {
        Ok(Self {
            score: if passing { 1. } else { 0. },
            max_score: 1.,
            name: name,
            label: None,
            output: Some(output),
            tags: None,
            visibility: None,
        })
    }
    pub fn coverage_result(
        coverage: f32,
        score: f32,
        name: String,
        output: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            score: score * coverage,
            max_score: score,
            name,
            label: None,
            output: output,
            tags: None,
            visibility: None,
        })
    }
}
