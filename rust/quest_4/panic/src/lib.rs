use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).expect("File not found")
}