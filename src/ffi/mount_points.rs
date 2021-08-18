use std::{
    ffi::{CStr, CString, OsStr},    
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use libc::{getmntent, setmntent};
use crate::error::{Error, Result};

#[derive(Debug)]
pub struct MountPoint {
    pub fs_name: String,
    pub fs_path_prefix: PathBuf,
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
