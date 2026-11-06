use std::collections::HashSet;

pub fn unique_letters(text: &str) -> (String, usize) {
    let letters: Vec<char> = text.chars().collect();
    let unique: HashSet<u8> = text.bytes().collect();
    let same: String = letters.iter().collect();
    (same, unique.len())
}
