mod ffi;

use std::{env, ffi::OsString, path::PathBuf};

use rayon::prelude::*;

fn remove_file(to_be_removed: &OsString) {
    // to_be_removed.par_iter().map(|x|)
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(home_dir) = std::env::var("HOME") {
        Some(PathBuf::from(home_dir))
    } else {
        ffi::get_home_dir()
    }
}

/// The path of the home trash directory, as specified by FreeDesktop's trash-spec 1.0
/// Ref.: https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html
fn home_trash_path() -> PathBuf {
    if let Ok(xdg_home) = std::env::var("XDG_DATA_HOME") {
        return PathBuf::from(xdg_home);
    }
    
    let home_dir = home_dir().expect("failed to obtain user's home directory!");

    home_dir.join(".local/share/Trash")
}

fn main() {
    let to_be_removed: Vec<_> = env::args_os().skip(1).collect();

    dbg!(home_trash_path());

    // to_be_removed.par_iter().map(|x)
}
