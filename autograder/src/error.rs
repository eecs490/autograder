use clap;
use lcov;
use serde_json;
use serde_yaml;
use std::error;
use std::fmt;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    IOError(String),
    FromUtf8Error(FromUtf8Error),
    LcovReaderError(lcov::reader::Error),
    ScoreError(String),
    ClapError(clap::Error),
}

impl Error {
    pub fn io_error_from(err: io::Error, path: &Path) -> Self {
        Self::IOError(format!("{} {:?}", err, path))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonError(ref e) => e.fmt(f),
            Error::YamlError(ref e) => e.fmt(f),
            Error::IOError(ref e) => e.fmt(f),
            Error::FromUtf8Error(ref e) => e.fmt(f),
            Error::LcovReaderError(ref e) => e.fmt(f),
            Error::ScoreError(ref e) => e.fmt(f),
            Error::ClapError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::JsonError(ref e) => Some(e),
            Error::YamlError(ref e) => Some(e),
            Error::IOError(_) => None,
            Error::FromUtf8Error(ref e) => Some(e),
            Error::LcovReaderError(_) => None,
            Error::ScoreError(_) => None,
            Error::ClapError(ref e) => Some(e),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::ClapError(err)
    }
}

impl From<lcov::reader::Error> for Error {
    fn from(err: lcov::reader::Error) -> Error {
        Error::LcovReaderError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::YamlError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUtf8Error(err)
    }
}
