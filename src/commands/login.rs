use super::Command;
use crate::cli::login::Login;
use crate::config::AppConfig;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use std::io::Write;

#[async_trait]
impl Command for Login {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        print!("Enter your passphrase: ");
        std::io::stdout().flush()?;
        let passphrase = rpassword::read_password()?;
        let encrypted_token = crate::config::Token::encrypt(&self.token, &passphrase)?;

        let encoded = general_purpose::STANDARD_NO_PAD.encode(encrypted_token);

        let new_cfg = AppConfig {
            e_token: Some(encoded),
            ..cfg
        };

        confy::store(crate::APP_NAME, None, new_cfg)?;

        Ok(())
    }
}
