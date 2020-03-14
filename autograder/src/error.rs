use snafu::ResultExt;
use snafu::Snafu;
use std::{io, path::PathBuf};

#[derive(Debug, Snafu)]
pub enum MyError {
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: io::Error, path: PathBuf },

    #[snafu(display("Unable to write result to {}: {}", path.display(), source))]
    WriteResult { source: io::Error, path: PathBuf },
}

type Result<T, E = MyError> = std::result::Result<T, E>;
