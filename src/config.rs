use crate::provider::Provider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub provider: Option<Provider>,
    pub e_token: Option<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Self { token }
    }
    pub fn encrypt(&self) -> Result<String, crate::error::Error> {
        todo!()
    }
    pub fn decrypt(&self) -> Result<String, crate::error::Error> {
        todo!()
    }
}
