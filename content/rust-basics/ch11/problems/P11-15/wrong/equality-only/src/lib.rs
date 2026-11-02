#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(scalar_len("abc"), 3, "three ASCII letters");
        assert_eq!(scalar_len("éé"), 2, "two characters, four bytes");
        assert_eq!(scalar_len(""), 0, "empty text");
        assert_eq!(scalar_len("e\u{301}"), 2, "a letter and a combining accent are two characters");
    }
}
