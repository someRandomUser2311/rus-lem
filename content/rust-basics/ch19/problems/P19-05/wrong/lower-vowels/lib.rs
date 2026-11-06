pub fn classify(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'a'..='z' | 'A'..='Z' => "consonant",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' => "space",
        _ => "other",
    }
}

pub fn grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    }
}
