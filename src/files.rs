use std::{fmt::format, fs, path};

pub fn test_create_dir() {
    let folder = "joey";
    let my_path = path::Path::new(folder);
    if my_path.exists() {
        println!("Already exits folder");
        return;
    }
    let a = fs::create_dir("joey");
}

pub fn file_create() {
    let folder = "./joey";
    let path = format!("{}/02.txt", folder);
    let find_id = path::Path::new(&path);
    if find_id.exists() {
        println!("Already exits");
        return;
    }
    let file = fs::File::create(path);
}

pub fn write_to_file() {
    let folder = "./joey";
    let path = format!("{}/02.txt", folder);
    let contents = "Hello JOey";
    let write_it = fs::write(path, contents);
    if write_it.is_ok() {
        println!("All good");
    }
}

pub fn delete_file() {
    let folder = "./joey";
    let path = format!("{}/02.txt", folder);
    let _a = fs::remove_file(path);
}

pub fn read_file() {
    let folder = "./joey";
    let path = format!("{}/01.txt", folder);
    let a = fs::read(path).unwrap();
    println!("{}", a[0]);
}
