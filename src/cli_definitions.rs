// use crate::git_helpers;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add current branch to favorites
    Add {
        key: String,
        branch_name: Option<String>,
    },
    /// Switch to a different branch
    Use { key: String },
    /// Remove favorite branch
    Del { key: String },
    /// Delete all favorite branches
    DelAll,
    /// Print Branch Name
    Branch { key: Option<String> },
    /// Create a new branch that is named with a value for a given key
    New { key: String },
    /// List all favorite branches
    Ls,
}
