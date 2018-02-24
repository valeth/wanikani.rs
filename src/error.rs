use std::result;
use serde_json::Error as JSONError;
use reqwest::Error as ReqwestError;
use reqwest::UrlError as ReqwestUrlError;

#[derive(Debug)]
pub enum Error {
    JSONError(JSONError),
    RetrievalError(ReqwestError),
    URLParseError(ReqwestUrlError)
}

impl From<JSONError> for Error {
    fn from(error: JSONError) -> Self {
        Error::JSONError(error)
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::RetrievalError(error)
    }
}

impl From<ReqwestUrlError> for Error {
    fn from(error: ReqwestUrlError) -> Self {
        Error::URLParseError(error)
    }
}

pub type Result<T> = result::Result<T, Error>;

