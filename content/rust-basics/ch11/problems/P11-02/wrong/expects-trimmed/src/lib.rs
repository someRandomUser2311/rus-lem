#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_a_word() {
        assert_eq!(bracket("rust"), "[rust]");
    }

    #[test]
    fn trims_spaces_around_the_text() {
        assert_eq!(bracket("  a b "), "[a b]");
    }

    #[test]
    fn brackets_empty_text() {
        assert_eq!(bracket(""), "[]");
    }

    #[test]
    fn keeps_every_line() {
        assert_eq!(bracket("one\ntwo"), "[one\ntwo]");
    }

    #[test]
    fn keeps_accents_and_emoji() {
        assert_eq!(bracket("cafe\u{301} 😀"), "[cafe\u{301} 😀]");
    }
}
