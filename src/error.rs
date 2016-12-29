use std::io;
use hyper;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    HttpError(hyper::error::Error),
    IoError(io::Error),
    DecoderError(json::DecoderError),
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

impl From<json::DecoderError> for Error {
    fn from(e: json::DecoderError) -> Self {
        Error::DecoderError(e)
    }
}
