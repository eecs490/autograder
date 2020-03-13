use crate::error::ResultExt;
use crate::error::{failed_to_read, Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::iter::once;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreMap {
    pub line_coverage: f32,
    pub their_tests: f32,
    pub our_tests: BTreeMap<String, f32>,
}

impl ScoreMap {
    pub fn our_test_names(&'_ self) -> impl Iterator<Item = String> + '_ {
        self.our_tests.keys().cloned()
    }

    pub fn values(&'_ self) -> impl Iterator<Item = f32> + '_ {
        self.our_tests
            .values()
            .copied()
            .chain(once(self.line_coverage))
            .chain(once(self.their_tests))
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let string = fs::read_to_string(path).chain_err(|| failed_to_read(&path))?;
        let msg = format!(
            "\
Failed to convert the following string to struct ScoreMap:

{}",
            string
        );
        serde_yaml::from_str(&string).chain_err(|| msg)
    }

    pub fn get(&self, name: &String) -> Result<f32> {
        match self.our_tests.get(name) {
            None => Err(Error::from(ErrorKind::ScoreError(name.clone()))),
            Some(x) => Ok(*x),
        }
    }
}
