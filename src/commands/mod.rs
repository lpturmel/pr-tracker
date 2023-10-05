use async_trait::async_trait;

pub mod account;
pub mod login;

/// Trait for defining execution of a command
#[async_trait]
pub trait Command {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error>;
}
