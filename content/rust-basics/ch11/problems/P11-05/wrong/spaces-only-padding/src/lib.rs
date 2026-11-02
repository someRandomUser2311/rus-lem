#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_a_plain_tag() {
        assert_eq!(tag("Rust"), Ok(String::from("rust")), "a plain tag is lowercased");
    }

    #[test]
    fn trims_spaces() {
        assert_eq!(tag("  Rust  "), Ok(String::from("rust")), "spaces are trimmed");
    }

    #[test]
    fn eight_characters_fit_and_nine_do_not() {
        assert_eq!(tag("abcdefgh"), Ok(String::from("abcdefgh")), "8 characters fit");
        assert_eq!(tag("abcdefghi"), Err(TagError::TooLong), "9 characters are too long");
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(tag("東京タワー"), Ok(String::from("東京タワー")), "5 characters, 15 bytes");
    }

    #[test]
    fn lowercases_non_ascii_letters() {
        assert_eq!(tag("ÉTÉ"), Ok(String::from("été")), "É lowercases to é");
    }

    #[test]
    fn empty_and_blank_text_are_empty() {
        assert_eq!(tag(""), Err(TagError::Empty), "empty text");
        assert_eq!(tag("   "), Err(TagError::Empty), "spaces only");
    }
}
