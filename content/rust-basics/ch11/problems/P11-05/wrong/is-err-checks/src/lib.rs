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
        assert!(tag("abcdefghi").is_err(), "9 characters are too long");
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
        assert!(tag("").is_err(), "empty text");
        assert!(tag(" \t ").is_err(), "whitespace only");
    }
}
