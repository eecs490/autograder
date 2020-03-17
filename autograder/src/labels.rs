use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreMap {
    pub line_coverage: String,
    pub our_tests: String,
    pub their_tests: String,
}
