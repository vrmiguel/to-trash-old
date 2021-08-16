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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use crate::ffi::Stat;
    use crate::move_file;
    use crate::test::dummy_bytes;

    #[test]
    fn test_clone_and_delete() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path();

        let contents = dummy_bytes();

        let file_path = dir_path.join("dummy");
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&contents).unwrap();
        }
        assert!(file_path.exists());

        let prev_stat = Stat::lstat(&file_path).unwrap();

        let new_path = dir_path.join("moved_dummy");
        // There shouldn't be anything here yet
        assert!(!new_path.exists());
        move_file::clone_and_delete(&file_path, &new_path).unwrap();

        // This file shouldn't exist anymore!
        assert!(!file_path.exists());
        // And this one should now exist
        assert!(new_path.exists());

        let new_stat = Stat::lstat(&new_path).unwrap();

        assert_eq!(contents, std::fs::read(new_path).unwrap());

        // Make sure that permission bits, accessed & modified times were maintained
        assert_eq!(prev_stat.permissions(), new_stat.permissions());

        assert_eq!(prev_stat.modified(), new_stat.modified());

        assert_eq!(prev_stat.accessed(), new_stat.accessed());
    }

    #[test]
    fn test_move_file() {
        let dir = tempfile::tempdir().unwrap();
        let dir_path = dir.path();

        let contents = dummy_bytes();

        let file_path = dir_path.join("dummy");
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&contents).unwrap();
        }
        assert!(file_path.exists());

        let prev_stat = Stat::lstat(&file_path).unwrap();

        let new_path = dir_path.join("moved_dummy");
        // There shouldn't be anything here yet
        assert!(!new_path.exists());
        move_file::move_file(&file_path, &new_path).unwrap();

        // This file shouldn't exist anymore!
        assert!(!file_path.exists());
        // And this one should now exist
        assert!(new_path.exists());

        let new_stat = Stat::lstat(&new_path).unwrap();

        assert_eq!(contents, std::fs::read(new_path).unwrap());

        // Make sure that permission bits, accessed & modified times were maintained
        assert_eq!(prev_stat.permissions(), new_stat.permissions());

        assert_eq!(prev_stat.modified(), new_stat.modified());

        assert_eq!(prev_stat.accessed(), new_stat.accessed());
    }
}
