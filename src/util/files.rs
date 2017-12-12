use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;

pub fn read_file_to_string(file_path: &str) -> String {
    let mut path_buf = PathBuf::from(env::current_dir().unwrap());
    path_buf.push(file_path);

    let path = path_buf.as_path();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}\n", path.display(), why.description()),
        Ok(file) => file
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => {
            return contents
        }
    }
}