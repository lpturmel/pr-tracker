use super::Command;
use crate::cli::account::Account;

impl Command for Account {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        let email = cfg.email.ok_or(crate::error::Error::Unauthorized)?;
        println!("Signed in as: {}", email);

        Ok(())
    }
}
