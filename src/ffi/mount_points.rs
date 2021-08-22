use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    ffi::{CStr, CString, OsStr},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use crate::error::{Error, Result};
use libc::{getmntent, setmntent};

#[derive(Debug, PartialEq, Eq)]
pub struct MountPoint {
    pub fs_name: String,
    pub fs_path_prefix: PathBuf,
}

impl PartialOrd for MountPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for MountPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fs_path_prefix
            .as_os_str()
            .len()
            .cmp(&other.fs_path_prefix.as_os_str().len())
    }
}

pub fn probe_mount_points() -> Result<Vec<MountPoint>> {
    let mut mount_points = BinaryHeap::new();

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
        mount_points.push(Reverse(mount_point));
    }

    Ok(mount_points
        .into_sorted_vec()
        .into_iter()
        .map(|rev_mount_point| rev_mount_point.0)
        .collect())
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

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

        assert!(Reverse(first) > Reverse(second))
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

    #[test]
    fn probing_returns_ordered_mount_points() {
        let mount_points = super::probe_mount_points().unwrap();

        if mount_points.len() < 2 {
            // We didn't get enough data in order to test this :C
            //
            // TODO: check if it's possible to mock `probe_mount_points`.
            return;
        }

        assert!(mount_points.windows(2).all(|w| w[0] >= w[1]));
    }
}
