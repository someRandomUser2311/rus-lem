use std::fs;
use std::io;

pub fn line_count(path: &str) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut n = 0;
    for c in contents.chars() {
        if c == '\n' {
            n += 1;
        }
    }
    Ok(n)
}
