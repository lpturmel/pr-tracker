use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to read config file")]
    ConfigError(confy::ConfyError),
}

impl From<confy::ConfyError> for Error {
    fn from(err: confy::ConfyError) -> Self {
        Error::ConfigError(err)
    }
}
