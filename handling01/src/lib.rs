// handling01/src/lib.rs
use std::fs::{ OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut fd = OpenOptions::new().append(true)
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .expect("Failed to open file ");
    fd.write(content.as_bytes())
        .expect("Failed to whrite to file ");
}
