use clap;
use serde_yaml;
use snafu::Snafu;
use std::{io, option, path::PathBuf};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MyError {
    #[snafu(display("Failed to read from {}: {}", path.display(), source))]
    ReadError { source: io::Error, path: PathBuf },

    #[snafu(display("Unable to parse yaml to ScoreMap:\n{}\n{}", yaml, source))]
    ScoreMapParseError {
        source: serde_yaml::Error,
        yaml: String,
    },

    #[snafu(display("Key {} not found in ScoreMap: ", key))]
    ScoreMapKeyError { key: String },

    #[snafu(display("Bad arg {}: {}", arg, source))]
    Argument { source: clap::Error, arg: String },
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;
