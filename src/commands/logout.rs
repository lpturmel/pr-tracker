use super::Command;
use crate::cli::logout::Logout;
use crate::config::AppConfig;
use crate::emoji::SPARKLES;
use async_trait::async_trait;

#[async_trait]
impl Command for Logout {
    async fn execute(&self, _cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        let new_cfg = AppConfig {
            token: None,
            provider: None,
            email: None,
        };
        confy::store(crate::APP_NAME, None, new_cfg)?;

        println!("{} Successfully logged out", SPARKLES);

        Ok(())
    }
}
