//! Content-addressed database for objects.
//! My idea was to use SQLite for this but using the fs will be easier at first so I'm gonna do that lol.
//!
//! Currently, the database is structured like this:
//!
//! ```text
//! .fuf
//! |-- db
//!     |-- objects
//!         |-- 0d
//!         |   |-- 0da893381fdb97c73eb9ca8c68fb0a04803f1f17d8c72a72c84b9535f62c08cd
//!         |-- 60
//!             |-- 60d4301d00238a934f94eacd0d4963ca7810afd9f198256e9f6aea2d8c101793
//! ```

use std::io::{self, Read};
use std::{fmt::Display, io::Write};

use cap_std::fs::Dir;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};

pub struct Db {
    objects: Dir,
}

#[derive(Debug, Clone, Copy)]
pub struct Address(blake3::Hash);

/// How often we bother retrying things when we race.
const RETRY_TOLERANCE: usize = 5;

impl Db {
    pub fn init(dir: &Dir) -> Result<()> {
        dir.create_dir(".fuf/db").wrap_err("creating .fuf/db")?;
        dir.create_dir(".fuf/db/objects")
            .wrap_err("creating .fuf/db/objects")?;
        Ok(())
    }

    pub fn open(dir: &Dir) -> Result<Self> {
        let db = dir
            .open_dir(".fuf/db")
            .wrap_err("error opening .fuf/db in workspace")?;
        let objects = db
            .open_dir("objects")
            .wrap_err("error opening .fuf/db/objects")?;

        Ok(Self { objects })
    }

    pub fn save_file(&self, content: &[u8]) -> Result<Address> {
        let hash = Address(blake3::hash(content));

        let prefix_dir = self
            .open_prefix_dir(hash)
            .wrap_err("opening prefix dir for saving file")?;

        let suffix = &hash.0.to_hex()[2..];
        match prefix_dir.create(suffix) {
            Ok(mut file) => {
                file.write_all(content)
                    .wrap_err("writing contents of file")?;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                // Ok, it already exists. Do nothing.
                // But make sure it reaaaaally is what we want.
                if cfg!(debug_assertions) {
                    let mut existing_content = Vec::new();
                    let _: usize = prefix_dir
                        .open(suffix)
                        .wrap_err("opening file")?
                        .read_to_end(&mut existing_content)
                        .wrap_err("reading existing content")?;
                    if existing_content != content {
                        let _: Result<_, _> = std::fs::write(".fuf-inconsistenty-new!", content);
                        panic!("inconsistency file {hash} already exists with a different content. dumping new content to .fuf-inconsistenty-new!");
                    }
                }
            }
            Err(e) => return Err(e).wrap_err("opening file"),
        }

        Ok(hash)
    }

    fn open_prefix_dir(&self, hash: Address) -> Result<Dir> {
        let hash_string = hash.0.to_hex();
        let prefix = &hash_string[..2];

        open_or_create(
            || self.objects.open_dir(prefix),
            || self.objects.create_dir(prefix),
        )
    }
}

fn open_or_create<T>(
    open: impl Fn() -> io::Result<T>,
    create: impl Fn() -> io::Result<()>,
) -> Result<T> {
    for _ in 0..RETRY_TOLERANCE {
        let dir = open();

        match dir {
            Ok(dir) => return Ok(dir),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let dir = create();
                match dir {
                    Ok(()) => {
                        // Try opening again now that it's been created...
                    }
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                        // A race! Ok, try opening the other directory now...
                    }
                    Err(e) => return Err(e).wrap_err("failed to create file"),
                }
            }
            Err(e) => return Err(e).wrap_err("failed to open file"),
        }
    }
    bail!("repeated creations and deletions of file")
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}
