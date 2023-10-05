use std::string::FromUtf8Error;
use thiserror::Error;

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
