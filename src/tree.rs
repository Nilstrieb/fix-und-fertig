use std::io::Read;

use cap_std::fs::Dir;
use color_eyre::{
    eyre::{bail, Context, ContextCompat},
    Result,
};

use crate::{db::Address, workspace::Workspace};

pub(crate) struct Tree {
    pub(crate) children: Vec<TreeChild>,
}

pub(crate) struct TreeChild {
    address: Address,
    name: String,
    kind: TreeChildType,
}

pub(crate) enum TreeChildType {
    Dir,
    File,
}

pub(crate) fn save_tree(workspace: &Workspace, directory: &Dir) -> Result<Address> {
    let mut tree = vec![];

    for entry in directory.entries().wrap_err("reading directory")? {
        let entry = entry.wrap_err("reading directory")?;

        let filetype = entry.file_type().wrap_err("getting entry file type")?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().wrap_err("non-UTF-8 file name")?;
        if file_name.contains(['\n', '\r']) {
            bail!("file name {file_name} contains a newline, which is not supported");
        }

        match () {
            () if filetype.is_dir() => {
                let inner_dir = entry.open_dir().wrap_err("opening directory")?;
                let inner_address = save_tree(workspace, &inner_dir)
                    .wrap_err_with(|| format!("saving directory {file_name}"))?;
                tree.push(TreeChild {
                    address: inner_address,
                    name: file_name.to_owned(),
                    kind: TreeChildType::Dir,
                });
            }
            () if filetype.is_file() => {
                let wrap_err = || format!("reading {file_name}");
                let mut file = entry.open().wrap_err_with(wrap_err)?;
                let mut content = vec![];
                file.read_to_end(&mut content).wrap_err_with(wrap_err)?;
                let address = workspace
                    .db
                    .save_blob(&content)
                    .wrap_err("saving file to database")?;
                tree.push(TreeChild {
                    address,
                    name: file_name.to_owned(),
                    kind: TreeChildType::File,
                });
            }
            _ => {
                bail!("file {file_name} with type {filetype:?} is not supported")
            }
        }
    }

    let mut content = vec![];
    Tree { children: tree }.serialize(&mut content);

    workspace
        .db
        .save_blob(&content)
        .wrap_err("saving tree blob")
}

impl Tree {
    pub(crate) fn serialize(&self, out: &mut Vec<u8>) {
        use std::io::Write;
        for child in &self.children {
            write!(
                out,
                "{}",
                match child.kind {
                    TreeChildType::Dir => "D",
                    TreeChildType::File => "F",
                }
            )
            .unwrap();
            write!(out, " {}", child.address).unwrap();
            write!(out, " {}", child.name).unwrap();
            write!(out, "\n").unwrap();
        }
    }
}
