use crate::error::ReadError;
use crate::score_map::ScoreMap;
use crate::Result;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::fs;
use std::path::Path;
//{ "type":"suite", "event": "started", "test_count": 5 }
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Suite,
    Test,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Ok,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestOutput {
    #[serde(alias = "type")]
    pub _type: Type,
    pub name: String,
    pub event: Event,
    pub stdout: Option<String>,
    pub message: Option<String>,
    pub score: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct TestOutputs(Vec<TestOutput>);

impl TestOutput {
    pub fn passing(&self) -> bool {
        match self.event {
            Event::Ok => true,
            Event::Failed => false,
        }
    }
    pub fn assign_score(&self, score: f32) -> Self {
        Self {
            score: Some(score),
            ..self.clone()
        }
    }
}
use std::cmp::Ordering;

impl TestOutputs {
    pub fn from_output(test_output: String) -> Self {
        Self(
            test_output
                .split("\n")
                .map(serde_json::from_str)
                .filter_map(std::result::Result::ok)
                .collect(),
        )
    }
    pub fn from_path(path: &Path) -> Result<Self> {
        let utf8: Vec<u8> = fs::read(path).context(ReadError { path })?;
        let output = String::from_utf8_lossy(&utf8).into_owned();
        Ok(Self::from_output(output))
    }

    pub fn assign_scores(&self, scores: &ScoreMap) -> Self {
        Self(
            self.0
                .iter()
                .map(|r| r.assign_score(scores.their_tests))
                .collect(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn names(&self) -> impl Iterator<Item = String> + '_ {
        self.0.iter().map(|r| r.name.clone())
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&TestOutput, &TestOutput) -> Ordering,
    {
        self.0.sort_by(compare)
    }
}

impl IntoIterator for TestOutputs {
    type Item = TestOutput;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
