use super::Command;
use crate::cli::login::Login;
use async_trait::async_trait;

#[async_trait]
impl Command for Login {
    async fn execute(&self, config: &crate::config::AppConfig) -> Result<(), crate::error::Error> {
        println!("Login: {:?}", self);
        Ok(())
    }
}
