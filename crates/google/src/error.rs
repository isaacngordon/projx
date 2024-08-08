use std::fmt::{self, Display, Formatter};

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    ReqwestError(reqwest::Error),
    #[from]
    IoError(std::io::Error),
    #[from]
    SerdeJsonError(serde_json::Error),

    #[from]
    SystemTimeError(std::time::SystemTimeError),
    #[from]
    JsonWebTokenError(jsonwebtoken::errors::Error),

    CustomError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::ReqwestError(e) => write!(f, "ReqwestError: {}", e),
            Error::IoError(e) => write!(f, "IoError: {}", e),
            Error::SerdeJsonError(e) => write!(f, "SerdeJsonError: {}", e),
            Error::SystemTimeError(e) => write!(f, "SystemTimeError: {}", e),
            Error::JsonWebTokenError(e) => write!(f, "JsonWebTokenError: {}", e),
            Error::CustomError(e) => write!(f, "CustomError: {}", e),
        }
    }
}

// end boilerplate
