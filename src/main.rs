mod error;
mod ffi;
mod home;
mod move_file;

use core::panic;
use std::{env, ffi::OsString, fs, path::{Path, PathBuf}};

use lazy_static::lazy_static;
use rayon::prelude::*;

use crate::error::{Error, Result};

lazy_static! {
    pub static ref HOME_DIR: PathBuf =
        home::home_dir().expect("failed to obtain user's home directory!");
    pub static ref HOME_TRASH: PathBuf = home::home_trash_path();
    pub static ref HOME_TRASH_FILES: PathBuf = HOME_TRASH.join("files");
    pub static ref HOME_TRASH_SIZES: PathBuf = HOME_TRASH.join("directorysizes");
    pub static ref HOME_TRASH_INFO: PathBuf = HOME_TRASH.join("info");
}

/// Sends a file to trash
fn send_to_trash(to_be_removed: OsString) -> Result<()> {
    let path = PathBuf::from(to_be_removed);

    let parent = path.parent().unwrap_or(Path::new("/"));

    let canonicalized: PathBuf;

    let file_name = if !path.ends_with("..") && path != Path::new(".") { 
        path.file_name().unwrap()
    } else {
        canonicalized = fs::canonicalize(&path)?;
        match canonicalized.file_name() {
            Some(canon_path) => canon_path,
            None => return Err(Error::FailedToObtainFileName(path.clone())),
        }
    };

    if path.starts_with(&*HOME_DIR) {
        // TODO: check for preexisting file
        move_file::move_file(path.as_ref(), &*HOME_TRASH)?;
        
    } else {
        todo!("check for parent trash dir")
    }

    Ok(())
}

fn main() -> Result<()> {

    env::args_os().skip(1).map(send_to_trash).for_each(drop);

    Ok(())
}
