use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Provider {
    #[default]
    AzureDevOps,
    GitHub,
}
