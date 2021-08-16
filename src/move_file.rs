use std::fs;
use std::path::Path;

use crate::error::Result;

/// Will copy the contents of `from` into `to`.
/// `from` will then be deleted.
fn clone_and_delete(from: &Path, to: &Path) -> Result<()> {
    fs::copy(from, to)?;
    if from.is_dir() {
        fs::remove_dir_all(from)?;
    } else {
        fs::remove_file(from)?;
    }

    Ok(())
}

pub fn move_file(from: &Path, to: &Path) -> Result<()> {
    if let Err(_) = fs::rename(from, to) {
        // rename(2) failed, likely because the files are in different mount points
        // or are on separate filesystems.
        clone_and_delete(from, to)?;
    }

    Ok(())
}
