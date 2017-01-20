use std::io;
use std::error::Error as StdError;
use {hyper, serde_json};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ServerError(String),
    NotFound,
}

impl From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Self {
        Error::ServerError(e.description().to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ServerError(e.description().to_string())
    }
}
