use std::{ffi::OsString, fs::File, io::Write, path::Path};

use tempfile;

use rand::{rngs::SmallRng, RngCore, SeedableRng};

use crate::trash::make_unique_file_name;

fn dummy_bytes() -> Vec<u8> {
    let mut rng = SmallRng::from_entropy();
    let quantity = 1024 + rng.next_u32() % 1024;
    let mut vec = vec![0; quantity as usize];
    rng.fill_bytes(&mut vec);
    vec
}

// TODO: this test is really ugly
#[test]
fn make_unique_file_name_1() {
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
