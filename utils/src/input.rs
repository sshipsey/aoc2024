use std::fs;
use std::io::{self, BufRead};

pub fn read_to_string(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
