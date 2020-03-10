use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreMap {
    pub line_coverage: f32,
    pub branch_coverage: f32,
    pub their_tests: f32,
    pub our_tests: BTreeMap<String, f32>,
}

impl ScoreMap {
    pub fn values(&self) -> Vec<&f32> {
        let mut values: Vec<&f32> = self.our_tests.values().into_iter().collect();
        values.push(&self.line_coverage);
        values.push(&self.branch_coverage);
        values.push(&self.their_tests);
        values
    }
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let string = fs::read_to_string(path)?;
        serde_yaml::from_str(&string).map_err(|e| Error::YamlError(e))
    }

    pub fn get(&self, name: &String) -> Result<f32, Error> {
        match self.our_tests.get(name) {
            None => Err(Error::ScoreError(name.clone())),
            Some(x) => Ok(*x),
        }
    }
}
