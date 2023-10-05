use self::commands::Command;
use crate::emoji::{X, BANNER};
use clap::Parser;
use config::AppConfig;
use console::style;

mod azdo;
mod cli;
mod commands;
mod config;
mod emoji;
mod error;
mod provider;

const APP_NAME: &str = "pr-tracker";

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            let msg = style(format!("{} Error: {}", X, e)).red();
            eprintln!("{}", msg)
        }
    }
}

async fn run() -> Result<(), error::Error> {
    let cli = cli::Cli::parse();

    let cfg: AppConfig = confy::load(APP_NAME, None)?;

    println!("{}", BANNER);
    match cli.commands {
        cli::Commands::Login(login) => login.execute(cfg).await?,
        cli::Commands::Logout(logout) => logout.execute(cfg).await?,
        cli::Commands::Account(account) => account.execute(cfg).await?,
        cli::Commands::Pr(pr) => match pr.commands {
            cli::pr::PrCommands::Azdo(azdo) => azdo.execute(cfg).await?,
            cli::pr::PrCommands::Github(github) => github.execute(cfg).await?,
        },
    }
    Ok(())
}
