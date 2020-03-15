use crate::cargo_test_output::TestOutput;
use crate::error::Result;
use crate::score_map::ScoreMap;
use lcov::Record;
use serde::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct TestReport {
    #[serde(serialize_with = "to_str")]
    score: f32,
    max_score: f32,
    pub name: String,
    number: String,
    output: Option<String>,
    tags: Option<std::vec::Vec<String>>,
    visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn from_our_tests(result: &TestOutput, number: String, scores: &ScoreMap) -> Result<Self> {
        Ok(Self {
            score: if result.passing() {
                scores.get(&result.name.clone())?
            } else {
                0.
            },
            max_score: scores.get(&result.name.clone())?,
            name: result.name.clone(),
            number: number,
            output: result.stdout.clone().or(result.message.clone()),
            tags: None,
            visibility: None,
        })
    }
    pub fn from_their_tests(result: &TestOutput, number: String, score: f32) -> Result<Self> {
        Ok(Self {
            score: if result.passing() { score } else { 0. },
            max_score: score,
            name: result.name.clone(),
            number: number,
            output: result.stdout.clone().or(result.message.clone()),
            tags: None,
            visibility: None,
        })
    }
    pub fn line_coverage(
        records: &Vec<Record>,
        number: String,
        score: f32,
        output: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            score: score * line_coverage(records),
            max_score: score,
            name: "line coverage".into(),
            number: number,
            output: output,
            tags: None,
            visibility: None,
        })
    }

    #[allow(dead_code)]
    pub fn branch_coverage(
        records: &Vec<Record>,
        number: String,
        score: f32,
        output: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            score: score * branch_coverage(records),
            max_score: score,
            name: "branch coverage".into(),
            number: number,
            output: output,
            tags: None,
            visibility: None,
        })
    }
}
