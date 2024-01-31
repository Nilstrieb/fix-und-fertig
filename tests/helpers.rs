use clap::{CommandFactory, FromArgMatches};
use color_eyre::Result;
use std::ffi::OsStr;

use fix_und_fertig::cli::Cli;

pub fn tmpdir() -> tempfile::TempDir {
    tempfile::TempDir::new().unwrap()
}

pub fn run_in<S: AsRef<OsStr>>(dir: &tempfile::TempDir, args: impl AsRef<[S]>) -> Result<()> {
    let mut matches = <Cli as CommandFactory>::command().get_matches_from(
        [OsStr::new("fuf")]
            .into_iter()
            .chain(args.as_ref().into_iter().map(AsRef::as_ref)),
    );
    let cli =
        <Cli as FromArgMatches>::from_arg_matches_mut(&mut matches).expect("invalid CLI args");

    fix_und_fertig::cli::run_command(dir.path(), cli)
}
