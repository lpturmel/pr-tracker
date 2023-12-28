pub mod account;
pub mod login;
pub mod logout;
pub mod pr;

/// Trait for defining execution of a command
pub trait Command {
    async fn execute(&self, cfg: crate::config::AppConfig) -> Result<(), crate::error::Error>;
}
