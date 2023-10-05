use crate::provider::Provider;
use clap::Args;

#[derive(Debug, Args)]
pub struct Login {
    /// The personal access token to use
    #[arg(long, short)]
    pub token: String,

    #[arg(long, short, value_enum)]
    pub provider: Provider,
}
