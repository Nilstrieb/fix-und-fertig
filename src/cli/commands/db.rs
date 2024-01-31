//! `fuf db` commands.

use color_eyre::Result;
use std::{path::Path};

use crate::{cli::utils, workspace::Workspace};

pub fn save_file(workspace: &Workspace, path: &Path) -> Result<()> {
    let file = utils::read_file(path)?;

    Ok(())
}
