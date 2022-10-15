use std::fs::{File, OpenOptions};
use std::io::Read;
const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
pub struct Png {
    file_path: String,
    pub file: File,
}

impl Png {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_owned(),
            file: OpenOptions::new()
                .read(true)
                .open(file_path)
                .expect("Error in opening the png file."),
        }
    }

    pub fn is_png(&mut self) -> bool {
        let mut header: [u8; 8] = [0; 8];
        self.file
            .read(&mut header)
            .expect("Error in reading header from file.");
        if PNG_HEADER == header {
            return true;
        }
        false
    }
}

pub fn hello() {
    println!("hello from png");
}
