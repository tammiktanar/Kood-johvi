use std::fs::File;
use std::io::Write;



pub fn open_or_create(file: &str, content: &str) {
    let mut file = File::create(file).unwrap();
    file.write_all(content.as_bytes()).unwrap()
}