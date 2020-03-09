use crate::error::Error;
use crate::util::ScoreMap;
use serde::{Deserialize, Serialize};
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
    pub _type: Type,
    pub name: String,
    pub event: Event,
    pub stdout: Option<String>,
    pub message: Option<String>,
}

impl TestResult {
    pub fn get_score(&self, scores: &ScoreMap) -> Result<f32, Error> {
        match self.event {
            Event::Ok => {
                let score = scores.get(&self.name)?;
                Ok(*score)
            }
            Event::Failed => Ok(0.0),
        }
    }
    pub fn from_output(test_output: String) -> Vec<TestResult> {
        test_output
            .split("\n")
            .map(serde_json::from_str)
            .filter_map(Result::ok)
            .collect()
    }
}
