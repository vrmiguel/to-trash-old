use std::{
    ffi::{OsStr, OsString},
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use tempfile;

use rand::{rngs::SmallRng, RngCore, SeedableRng};

use crate::{
    ffi, info_file, move_file,
    trash::{self, make_unique_file_name, Trash},
    HOME_DIR,
};

pub fn dummy_bytes() -> Vec<u8> {
    let mut rng = SmallRng::from_entropy();
    let quantity = 1024 + rng.next_u32() % 1024;
    let mut vec = vec![0; quantity as usize];
    rng.fill_bytes(&mut vec);
    vec
}

#[test]
fn test_make_info_file_path() {
    let trash_info = Path::new("/user/dummy/.local/share/Trash/info");
    let file_name = OsStr::new("deleted-file");

    assert_eq!(
        info_file::make_info_file_path(file_name, trash_info),
        PathBuf::from("/user/dummy/.local/share/Trash/info/deleted-file.trashinfo")
    );
}

#[test]
fn test_build_info_file() {
    let dir = tempfile::tempdir_in(&*HOME_DIR).unwrap();
    // let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path();
    let trash = Trash::new(dir_path);

    fs::create_dir(&trash.info).unwrap();

    let file_name = OsString::from("dummy");
    let dummy_file_path = dir_path.join(&file_name);
    let mut dummy = File::create(&dummy_file_path).unwrap();
    dummy.write_all(&dummy_bytes()).unwrap();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    info_file::build_info_file(&dummy_file_path, &file_name, &trash, now).unwrap();

    let info_file_path = trash.info.join("dummy.trashinfo");
    let info_file = fs::read_to_string(&info_file_path).unwrap();

    let rfc3339 = ffi::format_time(now).unwrap();

    let info_file_should_be = format!(
        "[Trash Info]\nPath={}\nDeletionDate={}\n",
        dummy_file_path.display(),
        rfc3339
    );

    assert_eq!(info_file, info_file_should_be)
}

#[test]
fn test_send_to_trash() {
    let dir = tempfile::tempdir_in(&*HOME_DIR).unwrap();
    let dir_path = dir.path();
    let trash = Trash::new(dir_path);

    fs::create_dir(&trash.directory_sizes).unwrap();
    fs::create_dir(&trash.files).unwrap();
    fs::create_dir(&trash.info).unwrap();

    let dummy_path = dir_path.join("dummy");
    let mut dummy = File::create(&*dummy_path).unwrap();
    dummy.write_all(&dummy_bytes()).unwrap();

    trash::send_to_trash(dummy_path.as_os_str().to_os_string(), &trash).unwrap();

    // This path should no longer exist!
    assert!(!dummy_path.exists());

    // The file should now be in the trash
    let new_path = trash.files.join("dummy");
    dbg!(&new_path);

    // The new file (now in the trash) should now exist
    assert!(new_path.exists());
}

#[test]
fn rfc3339_formatting() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("it seems that time went backwards!");

    use chrono::Local;

    // We'll use the chrono crate to make sure that
    // our own formatting (done through libc's strftime) works
    let date_time = Local::now();

    // YYYY-MM-DDThh:mm:ss
    let rfc3339 = date_time.format("%Y-%m-%dT%T").to_string();

    assert_eq!(&rfc3339, &ffi::format_time(now).unwrap());
}

// TODO: this test could look better
#[test]
fn test_make_unique_file_name() {
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path();

    let filename = Path::new("foo");

    let foo_1 = dir_path.join(filename);
    let mut file = File::create(&foo_1).unwrap();
    file.write_all(&dummy_bytes()).unwrap();
    assert!(foo_1.exists());

    let new_file_name = make_unique_file_name(filename, dir_path);
    assert_eq!(new_file_name, OsString::from("foo-1"));

    let foo_2 = dir_path.join("foo-1");
    let mut file = File::create(&foo_2).unwrap();
    file.write_all(&dummy_bytes()).unwrap();
    assert!(foo_2.exists());

    let new_file_name = make_unique_file_name(filename, dir_path);
    assert_eq!(new_file_name, OsString::from("foo-2"));

    println!("{:?}", new_file_name);
}
