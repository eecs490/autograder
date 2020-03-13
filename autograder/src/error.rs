mod error {
    error_chain! {}
}

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
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
        //Error::from(match err {
        //Io(e) => ErrorKind::Io(e),
        //ParseRecord(_, _) => panic!("oh shit"), // ErrorKind::LcovReaderError(err),
        //})
    }
}
