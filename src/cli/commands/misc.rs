use cap_std::fs::Dir;
use color_eyre::Result;

use crate::workspace::Workspace;

pub fn init(dir: &Dir) -> Result<()> {
    Workspace::init(dir)
}
