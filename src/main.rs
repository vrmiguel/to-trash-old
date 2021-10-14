mod error;
mod ffi;
mod home;
mod info_file;
mod move_file;
mod trash;

#[cfg(test)]
mod test;

use std::{
    env,
    path::{Path, PathBuf},
};

use error::{Error, Result};
use ffi::MountPoint;
use lazy_static::lazy_static;
use trash::Trash;

use crate::ffi::probe_mount_points;

lazy_static! {
    pub static ref HOME_DIR: PathBuf =
        home::home_dir().expect("failed to obtain user's home directory!");
    pub static ref HOME_TRASH_ROOT: PathBuf = home::home_trash_path();
    pub static ref HOME_TRASH: Trash = Trash::new(&HOME_TRASH_ROOT);
    pub static ref MOUNT_POINTS: Vec<MountPoint> =
        ffi::probe_mount_points().expect("Failed to probe mount points!");
}

fn mount_point_of_file(path: &Path) -> Option<&MountPoint> {
    for mount_point in MOUNT_POINTS.iter() {
        if mount_point.contains(path) {
            return Some(mount_point);
        }
    }

    None
}

fn main() -> Result<()> {

    for file in env::args_os().skip(1) {
        let file = PathBuf::from(file).canonicalize()?;

        let mount_point = mount_point_of_file(file.as_ref())
            .ok_or_else(|| Error::MountPointNotFound(file.clone()))?;

        let is_home = file.starts_with("/home") || mount_point.is_home();

        if is_home {
            trash::send_to_trash(file, &HOME_TRASH)?
        } else {
            // TODO: buncha stuff
            let trash = Trash::new(&mount_point.fs_path_prefix);
            trash::send_to_trash(file, &trash)?
        }
    }

    Ok(())
}
