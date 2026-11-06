use std::fs;
use std::io;

pub fn line_count(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path).unwrap();
    Ok(contents.lines().count())
}
