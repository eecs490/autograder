use snafu::{Backtrace, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not open config from {}: {}", filename.display(), source))]
    OpenConfig {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not save config to {}: {}", filename.display(), source))]
    SaveConfig {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("The user id {} is invalid", user_id))]
    UserIdInvalid { user_id: i32, backtrace: Backtrace },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
