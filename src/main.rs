mod error;
mod ffi;
mod home;
mod move_file;
mod trash;

use std::{env, path::{Path, PathBuf}};

use lazy_static::lazy_static;

use crate::error::Result;

lazy_static! {
    pub static ref HOME_DIR: PathBuf =
        home::home_dir().expect("failed to obtain user's home directory!");
    pub static ref HOME_TRASH: PathBuf = home::home_trash_path();
    pub static ref HOME_TRASH_FILES: PathBuf = HOME_TRASH.join("files");
    pub static ref HOME_TRASH_SIZES: PathBuf = HOME_TRASH.join("directorysizes");
    pub static ref HOME_TRASH_INFO: PathBuf = HOME_TRASH.join("info");
}

fn main() -> Result<()> {
    env::args_os()
        .skip(1)
        .map(trash::send_to_trash)
        .for_each(drop);

    Ok(())
}
