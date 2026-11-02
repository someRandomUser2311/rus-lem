#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_keeps_bytes() {
        assert_eq!(clamp(0), 0, "the smallest byte");
        assert_eq!(clamp(7), 7, "an odd value in range");
        assert_eq!(clamp(255), 255, "the largest byte");
    }

    #[test]
    fn clamp_limits_values_outside_the_range() {
        assert_eq!(clamp(-1), 0, "just below the range");
        assert_eq!(clamp(i32::MIN), 0, "the most negative i32");
        assert_eq!(clamp(256), 255, "just above the range");
        assert_eq!(clamp(i32::MAX), 255, "the largest i32");
    }

    #[test]
    fn normalize_joins_words_with_single_spaces() {
        assert_eq!(normalize("a  b"), "a b", "a run of spaces");
        assert_eq!(
            normalize("  Rust Book  "),
            "Rust Book",
            "no space at either end"
        );
    }

    #[test]
    fn normalize_splits_on_unicode_whitespace() {
        assert_eq!(
            normalize("\tThe\u{3000}Rust\nBook"),
            "The Rust Book",
            "tab, U+3000 and line feed"
        );
        assert_eq!(normalize(" \t\u{3000} "), "", "whitespace only");
    }

    #[test]
    fn valid_accepts_1_to_100() {
        assert_eq!(valid(1), 1, "the smallest valid number");
        assert_eq!(valid(100), 100, "the largest valid number");
    }

    #[test]
    #[should_panic]
    fn valid_rejects_zero() {
        valid(0);
    }

    #[test]
    #[should_panic(expected = "at most one hundred")]
    fn valid_rejects_101() {
        valid(101);
    }

    #[test]
    fn parse_count_reads_padded_numbers() -> Result<(), String> {
        assert_eq!(parse_count(" 42 ")?, 42, "spaces are trimmed");
        assert_eq!(parse_count("4294967295")?, u32::MAX, "the largest count");
        Ok(())
    }

    #[test]
    fn parse_count_rejects_text_that_is_not_a_count() {
        assert_eq!(parse_count("x"), Err(String::from("invalid")), "letters");
        assert_eq!(
            parse_count("-1"),
            Err(String::from("invalid")),
            "a negative number"
        );
    }
}
