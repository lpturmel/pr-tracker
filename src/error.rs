use std::string::FromUtf8Error;
use thiserror::Error;

use crate::azdo::DevOpsError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config error: {0}")]
    Config(confy::ConfyError),

    #[error("Crypto error: {0}")]
    Crypto(ring::error::Unspecified),

    #[error("String error: {0}")]
    String(FromUtf8Error),

    #[error("IO error: {0}")]
    Io(std::io::Error),

    #[error("Base64 error: {0}")]
    Decode(base64::DecodeError),

    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Date parse error: {0}")]
    DateParse(chrono::ParseError),

    #[error("Not signed-in, run `pr-tracker login` to login")]
    Unauthorized,

    #[error("Azure DevOps error: {0}")]
    DevOps(DevOpsError),
}

impl From<DevOpsError> for Error {
    fn from(err: DevOpsError) -> Self {
        Error::DevOps(err)
    }
}
impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::DateParse(err)
    }
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}
impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Decode(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::String(err)
    }
}

impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Self {
        Error::Crypto(err)
    }
}
impl From<confy::ConfyError> for Error {
    fn from(err: confy::ConfyError) -> Self {
        Error::Config(err)
    }
}
