use crate::error::{ReadError, ScoreMapKeyError, ScoreMapParseError};
use crate::Result;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use snafu::ResultExt;
use std::collections::BTreeMap;
use std::fs;
use std::iter::once;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
        let string = fs::read_to_string(path).context(ReadError { path })?;
        let msg = format!(
            "\
Failed to convert the following string to struct ScoreMap:

{}",
            string
        );
        serde_yaml::from_str(&string).context(ScoreMapParseError { yaml: &string })
    }

    pub fn get(&self, key: &String) -> Result<f32> {
        Ok(*self.our_tests.get(key).context(ScoreMapKeyError { key })?)
    }
}
