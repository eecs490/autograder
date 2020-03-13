use std::path::Path;
mod error {
    error_chain! {}
}

error_chain! {
    foreign_links {
        Clap(::clap::Error);
        Yaml(::serde_yaml::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
    }
    errors {
        ScoreError(s: String) {
            display("Name not found in scores.yaml file: '{}'", s)
        }
        LcovReaderError(e: lcov::reader::Error) {
            display("Unable to read {}", e)
        }
    }
}

impl From<lcov::reader::Error> for Error {
    fn from(err: lcov::reader::Error) -> Error {
        Error::from(ErrorKind::LcovReaderError(err))
    }
}

pub fn failed_to_read(path: &Path) -> String {
    format!("Failed to read {}", path.display())
}
