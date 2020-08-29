use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    // Errors from crates
    ReqwestError(reqwest::Error),
    // Errors from this crate
    ScraperError(ScraperErrorType)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScraperErrorType {
    ElementNotFound,
    AttributeNotFound
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ReqwestError(ref err)  => err.fmt(f),
            Error::ScraperError(ref err) => write!(f, "Error occurred {:?}", err)
        }
    }
}
