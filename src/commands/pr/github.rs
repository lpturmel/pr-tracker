use crate::cli::pr::Github;
use crate::commands::Command;

impl Command for Github {
    async fn execute(&self, _cfg: crate::config::AppConfig) -> Result<(), crate::error::Error> {
        todo!();
    }
}
