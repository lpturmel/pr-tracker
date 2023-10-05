use std::io::Write;

use super::Command;
use crate::cli::account::Account;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};

#[async_trait]
impl Command for Account {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        print!("Enter your passphrase: ");
        std::io::stdout().flush()?;
        let passphrase = rpassword::read_password()?;
        if let Some(token) = cfg.e_token {
            let decode = general_purpose::STANDARD_NO_PAD.decode(token.as_bytes())?;
            let decrypted = crate::config::Token::decrypt(&decode, &passphrase)?;

            let azdo_client = crate::azdo::Client::new(&decrypted);
            let profile = azdo_client.get_profile().await?;

            println!(
                "\nSigned in as: {} ({})",
                profile.authenticated_user.provider_display_name,
                profile.authenticated_user.properties.account.value
            );
        }

        Ok(())
    }
}
