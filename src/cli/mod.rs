//! The command line interface and commands.

mod commands;
mod utils;

use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use cap_std::fs::Dir;
use clap::{Parser, Subcommand};
use color_eyre::{eyre::Context, Result};

use crate::{db::Address, workspace};

#[derive(Debug, Parser)]
#[command(name = "fuf")]
#[command(about = "The fix-und-fertig revision control system", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new fix-und-fertig workspace.
    Init {},
    /// Low-level commands for working with the .fuf database.
    Db {
        #[command(subcommand)]
        command: DbCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum DbCommand {
    /// Save a file into the database and get the hash.
    SaveFile {
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    /// Save a tree (directory) into the database and get the hash.
    SaveTree {
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    /// Reads the blob behind an address.
    ReadBlob {
        hash: Address,
    }
}

pub fn main() -> Result<()> {
    let cwd = env::current_dir().wrap_err("failed to get current directory")?;
    let cwd = cwd
        .canonicalize()
        .wrap_err("failed to canonicalize working directory. does it still exist?")?;

    let cli: Cli = Cli::parse();

    run_command(&cwd, cli)
}

pub fn run_command(cwd: &Path, cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init {} => {
            let cwd_file = File::open(&cwd)
                .wrap_err_with(|| format!("failed to open working directory {}", cwd.display()))?;
            let dir = Dir::from_std_file(cwd_file);
            commands::misc::init(&dir)
                .wrap_err_with(|| format!("creating fix-und-fertig workspace in {}", cwd.display()))
        }
        Command::Db { command } => {
            let workspace =
                workspace::Workspace::find(&cwd).wrap_err("failed to open workspace")?;

            match command {
                DbCommand::SaveFile { paths } => commands::db::save_file(&workspace, &paths),
                DbCommand::SaveTree { paths } => commands::db::save_tree(&workspace, &paths),
                DbCommand::ReadBlob { hash: address } => commands::db::read_blob(&workspace, address),
            }
        }
    }
}
