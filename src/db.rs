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

use cap_std::fs::Dir;
use color_eyre::{eyre::Context, Result};

pub struct Db {
    db: Dir,
    objects: Dir,
}

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

        Ok(Self { db, objects })
    }
}
