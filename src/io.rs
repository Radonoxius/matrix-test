use std::{fs::OpenOptions, io::Read, path::Path};

pub fn read_file(path: &Path) -> String {
    let mut buf = String::new();
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf
}