use clap::Parser;
use config::AppConfig;

use self::commands::Command;

mod azdo;
mod cli;
mod commands;
mod config;
mod error;
mod provider;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();

    let cfg: AppConfig = confy::load("pr-tracker", None)?;

    match cli.commands {
        cli::Commands::Login(login) => login.execute(&cfg).await?,
        cli::Commands::Logout(logout) => {
            println!("Logout: {:?}", logout);
        }
        cli::Commands::Account(account) => {
            println!("Account: {:?}", account);
        }
    }
    Ok(())
}
