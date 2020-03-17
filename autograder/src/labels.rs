use crate::error::{LabelsParseError, ReadError};
use crate::Result;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Labels {
    pub line_coverage: String,
    pub our_tests: String,
    pub their_tests: String,
}

impl Labels {
    pub fn from_path(path: &Path) -> Result<Self> {
        let yaml = fs::read_to_string(path).context(ReadError { path })?;
        serde_yaml::from_str(&yaml).context(LabelsParseError { yaml })
    }
}
