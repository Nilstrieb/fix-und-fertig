use cap_std::fs::Dir;
///! Utilities for the CLI functions. These should *not* be used outside of the CLI-specific code!
use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use std::{
    fmt::Display,
    fs::File,
    io::{self, Write},
    path::Path,
};

/// [`std::fs::read`], adding the path to the error message.
pub(super) fn read_file(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).wrap_err_with(|| format!("trying to open {}", path.display()))
}

pub fn dir_from_path(path: &Path) -> Result<Dir> {
    let dir = File::open(path).wrap_err_with(|| format!("opening directory {}", path.display()))?;
    if !dir.metadata()?.is_dir() {
        bail!("{} is not a directory", path.display());
    }
    Ok(Dir::from_std_file(dir))
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

/// Prints the bytes to stdout. Handles `BrokenPipe` by ignoring the rror.
/// Does *not* exit for `BrokenPipe`
pub(super) fn print_bytes(content: &[u8]) -> io::Result<()> {
    let result = std::io::stdout().lock().write_all(content);
    match result {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(e) => Err(e),
    }
}
