use lcov;
use locate_cargo_manifest::LocateManifestError;
use serde_json;
use std::error;
use std::fmt;
use std::io;
use std::option;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::Error),
    IOError(io::Error),
    FromUtf8Error(FromUtf8Error),
    ArgumentError(String),
    LcovReaderError(lcov::reader::Error),
    NoneError(option::NoneError),
    LocateManifestError(LocateManifestError),
}

impl Error {
    pub fn arg(msg: &str) -> Self {
        Error::ArgumentError(String::from(msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonError(ref e) => e.fmt(f),
            Error::IOError(ref e) => e.fmt(f),
            Error::FromUtf8Error(ref e) => e.fmt(f),
            Error::ArgumentError(ref e) => e.fmt(f),
            Error::LcovReaderError(ref e) => e.fmt(f),
            Error::NoneError(ref e) => write!(f, "NoneError: {:?}", e),
            Error::LocateManifestError(ref e) => write!(f, "LocateManifestError: {:?}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::JsonError(ref e) => Some(e),
            Error::IOError(ref e) => Some(e),
            Error::FromUtf8Error(ref e) => Some(e),
            Error::ArgumentError(_) => None,
            Error::LcovReaderError(_) => None,
            Error::NoneError(_) => None,
            Error::LocateManifestError(_) => None,
        }
    }
}

impl From<LocateManifestError> for Error {
    fn from(err: LocateManifestError) -> Error {
        Error::LocateManifestError(err)
    }
}

impl From<option::NoneError> for Error {
    fn from(err: option::NoneError) -> Error {
        Error::NoneError(err)
    }
}

impl From<lcov::reader::Error> for Error {
    fn from(err: lcov::reader::Error) -> Error {
        Error::LcovReaderError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUtf8Error(err)
    }
}
