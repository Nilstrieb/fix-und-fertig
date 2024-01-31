///! Utilities for the CLI functions. These should *not* be used outside of the CLI-specific code!
use color_eyre::{eyre::Context, Result};
use std::{
    fmt::Display,
    io::{self, Write},
    path::Path,
};

/// [`std::fs::read`], adding the path to the error message.
pub(super) fn read_file(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).wrap_err_with(|| format!("trying to open {}", path.display()))
}

/// Prints the content to stdout. Handles `BrokenPipe` by ignoring the rror.
/// Does *not* exit for `BrokenPipe`
pub(super) fn print(content: impl Display) -> io::Result<()> {
    let result = write!(std::io::stdout().lock(), "{}", content);
    match result {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e),
    }
}
