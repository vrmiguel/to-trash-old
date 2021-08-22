use std::{
    ffi::{CStr, CString, OsStr},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use crate::error::{Error, Result};
use libc::{getmntent, setmntent};

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct MountPoint {
    pub fs_name: String,
    pub fs_path_prefix: PathBuf,
}

impl PartialOrd for MountPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.fs_path_prefix
                .as_os_str()
                .len()
                .cmp(&other.fs_path_prefix.as_os_str().len()),
        )
    }
}

pub fn probe_mount_points() -> Result<Vec<MountPoint>> {
    let mut mount_points = vec![];

    let path = CString::new("/proc/mounts").unwrap();

    let read_arg = CString::new("r")?;
    let file = unsafe { setmntent(path.as_ptr(), read_arg.as_ptr()) };

    if file.is_null() {
        return Err(Error::FailedToObtainMountPoints);
    }

    loop {
        let entry = unsafe { getmntent(file) };
        if entry.is_null() {
            break;
        }
        // We just made sure `entry` is not null,
        // so this deref must be safe (I guess?)
        let fs_name = unsafe { (*entry).mnt_fsname };
        let fs_dir = unsafe { (*entry).mnt_dir };

        let fs_name_cstr = unsafe { CStr::from_ptr(fs_name) };
        let fs_name_cstr = OsStr::from_bytes(fs_name_cstr.to_bytes());
        let fs_name_str = String::from_utf8_lossy(fs_name_cstr.as_bytes());

        let fs_dir_cstr = unsafe { CStr::from_ptr(fs_dir) };
        let fs_dir_cstr = OsStr::from_bytes(fs_dir_cstr.to_bytes());
        let fs_dir_path = PathBuf::from(fs_dir_cstr);

        let mount_point = MountPoint {
            fs_name: fs_name_str.into(),
            fs_path_prefix: fs_dir_path,
        };
        mount_points.push(mount_point);
    }

    Ok(mount_points)
}

#[cfg(test)]
mod tests {
    use super::MountPoint;

    #[test]
    fn mount_point_cmp() {
        let first = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1000".into(),
        };

        let second = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1001/doc".into(),
        };

        assert!(first < second);
    }

    #[test]
    fn mount_point_neq() {
        // 1st case: same `fs_name` but differing prefix
        let first = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1000/doc".into(),
        };

        let second = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1001/doc".into(),
        };

        assert!(first != second);

        // 2nd case: differing `fs_name` but same prefix
        let first = MountPoint {
            fs_name: "portal2".into(),
            fs_path_prefix: "/run/user/1000/doc".into(),
        };

        let second = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1000/doc".into(),
        };

        assert!(first != second);

        // 3rd case: both properties differ
        let first = MountPoint {
            fs_name: "portal2".into(),
            fs_path_prefix: "/run/user/1000/doc".into(),
        };

        let second = MountPoint {
            fs_name: "portal".into(),
            fs_path_prefix: "/run/user/1001/doc".into(),
        };

        assert!(first != second);
    }
}
