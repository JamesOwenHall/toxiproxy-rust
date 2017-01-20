use std::io;
use {hyper, serde_json};

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::error::Error),
    IoError(io::Error),
    DecodeError(serde_json::Error),
    ServerError(String),
    NotFound,
}

impl From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Self {
        Error::HttpError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DecodeError(e)
    }
}
