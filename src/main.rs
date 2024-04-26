use std::{
    env,
    ffi::OsStr,
    fs::{copy, create_dir, read_dir, DirEntry},
    io::Error,
    path::PathBuf,
};

use dotenvy;

fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");

    let mut source_dir = String::from("");
    let mut dest_path = String::from("");

    for (key, value) in env::vars() {
        if key == "SOURCE_DIR" {
            source_dir = value.clone();
        }
        if key == "DEST_PATH" {
            dest_path = value;
        }
    }

    println!(
        "Starting sorting... Source: {:?}, Dest: {:?}",
        source_dir, dest_path
    );

    let dir = read_dir(source_dir);
    if let Err(err) = dir {
        panic!("Failed to read dir: {:?}", err);
    }
    let dir = dir.unwrap();

    let mut errors: Vec<Error> = vec![];

    let dir_v: Vec<Result<DirEntry, Error>> = dir.collect();

    let len = dir_v.len();
    println!("Found {:?} files", len);

    let mut current = 0;

    for item in dir_v {
        if let Err(err) = item {
            errors.push(err);
            return;
        }
        let item = item.unwrap();
        let src_path = item.path();
        let ext_name = src_path.extension();
        let ext_name = match ext_name {
            Some(v) => v,
            None => OsStr::new("NO_EXT"),
        };

        let filename = src_path.file_name();
        if let None = filename {
            continue;
        }
        let filename = filename.unwrap();

        let ext_dir_path = PathBuf::from(format!("{dest_path}/{}", ext_name.to_str().unwrap()));

        if !ext_dir_path.exists() {
            let res = create_dir(&ext_dir_path);
            if let Err(err) = res {
                panic!("Failed create ext dir: {:?}: {:?}", &ext_dir_path, err);
            }
        }

        let dest_path = PathBuf::from(format!(
            "{}/{}",
            ext_dir_path.to_str().unwrap(),
            filename.to_str().unwrap()
        ));

        current += 1;
        println!("{current}/{len} {:?}", &dest_path);

        if !dest_path.exists() {
            let res = copy(src_path, dest_path);
            if let Err(err) = res {
                errors.push(err);
            }
        }
    }

    println!("Sorting end with {} errors", errors.len());

    let mut num = 0;
    for error in errors {
        num += 1;
        println!("Error {}: {:?}", num, error);
    }
}
