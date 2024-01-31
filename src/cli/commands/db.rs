//! `fuf db` commands.

use color_eyre::{eyre::Context, Result};
use std::path::PathBuf;

use crate::{cli::utils, workspace::Workspace};

pub fn save_file(workspace: &Workspace, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        let file = utils::read_file(path)?;

        let address = workspace
            .db
            .save_file(&file)
            .wrap_err("saving file to database")?;

        utils::print(format_args!("{address}\n")).wrap_err("printing output")?;
    }

    Ok(())
}
