use self::commands::Command;
use clap::Parser;
use config::AppConfig;

mod azdo;
mod cli;
mod commands;
mod config;
mod error;
mod provider;

const APP_NAME: &str = "pr-tracker";

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e)
        }
    }
}

async fn run() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();

    let cfg: AppConfig = confy::load(APP_NAME, None)?;

    match cli.commands {
        cli::Commands::Login(login) => login.execute(cfg).await?,
        cli::Commands::Logout(logout) => {
            println!("Logout: {:?}", logout);
        }
        cli::Commands::Account(account) => account.execute(cfg).await?,
    }
    Ok(())
}
