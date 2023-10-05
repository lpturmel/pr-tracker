use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Pr {
    #[clap(subcommand)]
    pub commands: PrCommands,
}
#[derive(Debug, Subcommand)]
pub enum PrCommands {
    Azdo(Azdo),
    Github(Github),
}
#[derive(Debug, Args)]
pub struct Azdo {
    /// The project to use
    #[arg(long, short)]
    pub project: String,

    #[arg(long, short)]
    /// The organization to use
    pub organization: String,

    #[arg(long, short, value_parser, num_args = 1.., value_delimiter = ',')]
    /// The list of repositories to track
    pub repos: Vec<String>,
}
#[derive(Debug, Args)]
pub struct Github {
    /// The list of repositories to track
    #[arg(long, short)]
    pub repositories: String,

    #[arg(long, short)]
    /// The Github owner
    pub owner: String,
}
