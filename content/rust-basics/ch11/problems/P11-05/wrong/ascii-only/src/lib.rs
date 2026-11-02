#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_a_plain_tag() {
        assert_eq!(tag("Rust"), Ok(String::from("rust")), "a plain tag is lowercased");
    }

    #[test]
    fn trims_unicode_whitespace() {
        assert_eq!(tag("\t Rust\u{3000}\n"), Ok(String::from("rust")), "whitespace is trimmed");
    }

    #[test]
    fn eight_characters_fit_and_nine_do_not() {
        assert_eq!(tag("abcdefgh"), Ok(String::from("abcdefgh")), "8 characters fit");
        assert_eq!(tag("abcdefghi"), Err(TagError::TooLong), "9 characters are too long");
    }

    #[test]
    fn lowercases_shouting() {
        assert_eq!(tag("LOUD"), Ok(String::from("loud")), "capitals are lowercased");
    }

    #[test]
    fn keeps_inner_spaces() {
        assert_eq!(tag("a b"), Ok(String::from("a b")), "inner spaces stay");
    }

    #[test]
    fn empty_and_blank_text_are_empty() {
        assert_eq!(tag(""), Err(TagError::Empty), "empty text");
        assert_eq!(tag(" \t "), Err(TagError::Empty), "whitespace only");
    }
}
