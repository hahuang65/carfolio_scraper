use std::{error, fmt};

#[derive(Debug)]
pub enum AppError {
    // Errors from crates
    ReqwestError(reqwest::Error),
    // Errors from this crate
    StandardError(StandardErrorType),
    CustomError(String)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StandardErrorType {
    ElementNotFound,
    AttributeNotFound
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        AppError::ReqwestError(err)
    }
}

impl StandardErrorType {
    fn as_str(&self) -> &str {
        match *self {
            StandardErrorType::ElementNotFound   => "Element not found",
            StandardErrorType::AttributeNotFound => "Attribute not found"
        }
    }
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::ReqwestError(ref err)  => err.fmt(f),
            AppError::StandardError(ref err) => write!(f, "Error occurred {:?}", err),
            AppError::CustomError(ref err)   => write!(f, "Error occurred {:?}", err)
        }
    }
}
