use snafu;
use snafu::Snafu;
use std::option;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MyError {
    #[snafu(display("Key {} not found in ScoreMap: ", key))]
    NoneError {
        source: option::NoneError,
        key: String,
    },
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;
