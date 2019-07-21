use serde_json::error::Error as JsonError;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Program(&'static str),
    JsonError(JsonError),
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::Program(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::JsonError(err)
    }
}
