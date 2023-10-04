use clap::{Parser, Subcommand};

pub mod account;
pub mod login;
pub mod logout;

/// Pull Request tracker
#[derive(Parser, Debug)]
#[clap(author = "Louis-Philippe Turmel", version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login to the Azure Devops or GitHub account
    Login(login::Login),
    /// Logout from the Azure Devops or GitHub account
    Logout(logout::Logout),
    /// Get information about the currently logged in account
    Account(account::Account),
}
