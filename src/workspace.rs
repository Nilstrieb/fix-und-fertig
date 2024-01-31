use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::db::Db;
use cap_std::fs::Dir;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};

pub struct Workspace {
    _root: Dir,
    pub db: Db,
}

impl Workspace {
    pub fn init(dir: &Dir) -> Result<()> {
        dir.create_dir(".fuf").wrap_err("creating .fuf")?;
        Db::init(&dir)?;
        Ok(())
    }

    pub fn find(dir: &Path) -> Result<Self> {
        let Some(root) = find_workspace_dir(dir)? else {
            bail!("you are not in a fix-und-fertig workspace, indicated by a .fuf directory");
        };

        Self::open_root(&root).wrap_err_with(|| format!("opening workspace {}", root.display()))
    }

    pub fn open_root(root: &Path) -> Result<Self> {
        let dir = File::open(root).wrap_err("opening workspace directory")?;
        let dir = Dir::from_std_file(dir);
        let db = Db::open(&dir).wrap_err("opening .fuf database")?;
        Ok(Self { _root: dir, db })
    }
}

/// Finds the workspace that `dir` is part of.
pub fn find_workspace_dir(mut dir: &Path) -> Result<Option<PathBuf>> {
    assert!(dir.is_absolute());
    // No cap-std yet, we need to figure out the workspace first.
    loop {
        let readdir = std::fs::read_dir(dir)
            .wrap_err_with(|| format!("failed to read path {}", dir.display()))?;

        for child in readdir {
            let child = child
                .wrap_err_with(|| format!("failed to read entry in directory {}", dir.display()))?;
            if child.path().ends_with(".fuf") {
                return Ok(Some(dir.to_owned()));
            }
        }

        let Some(parent) = dir.parent() else {
            return Ok(None);
        };
        dir = parent;
    }
}
