mod error;
mod ffi;
mod home;
mod info_file;
mod move_file;
mod trash;

#[cfg(test)]
mod test;

use std::{env, path::PathBuf};

use lazy_static::lazy_static;
use trash::Trash;

use crate::error::Result;

lazy_static! {
    pub static ref HOME_DIR: PathBuf =
        home::home_dir().expect("failed to obtain user's home directory!");
    pub static ref HOME_TRASH_ROOT: PathBuf = home::home_trash_path();
    pub static ref HOME_TRASH: Trash = Trash::new(&HOME_TRASH_ROOT);
}

fn main() -> Result<()> {
    dbg!(ffi::probe_mount_points());
    env::args_os()
        .skip(1)
        .map(|file| trash::send_to_trash(file, &HOME_TRASH))
        .for_each(drop);

    Ok(())
}
