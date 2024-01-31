///! Utilities for the CLI functions. These should *not* be used outside of the CLI-specific code!

use color_eyre::{eyre::Context, Result};
use std::path::Path;

/// [`std::fs::read`], adding the path to the error message.
pub(super) fn read_file(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).wrap_err_with(|| format!("trying to open {}", path.display()))
}
