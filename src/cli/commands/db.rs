//! `fuf db` commands.

use color_eyre::{eyre::Context, Result};
use std::path::PathBuf;

use crate::{cli::utils, db::Address, workspace::Workspace};

pub fn save_file(workspace: &Workspace, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        let file = utils::read_file(path)?;

        let address = workspace
            .db
            .save_blob(&file)
            .wrap_err("saving file to database")?;

        utils::print(format_args!("{address}\n")).wrap_err("printing output")?;
    }

    Ok(())
}

pub fn save_tree(workspace: &Workspace, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        let dir = utils::dir_from_path(path)?;
        let address = crate::tree::save_tree(workspace, &dir)?;
        utils::print(format_args!("{address}\n")).wrap_err("printing output")?;
    }

    Ok(())
}

pub fn read_blob(workspace: &Workspace, address: Address) -> Result<()> {
    let content = workspace
        .db
        .read_blob(address)
        .wrap_err_with(|| format!("reading blog {address}"))?;

    utils::print_bytes(&content).wrap_err("printing bytes")?;
    Ok(())
}
