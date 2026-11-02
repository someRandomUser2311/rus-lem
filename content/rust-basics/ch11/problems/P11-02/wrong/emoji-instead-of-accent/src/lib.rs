#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_a_word() {
        assert_eq!(bracket("rust"), "[rust]");
    }

    #[test]
    fn keeps_spaces_around_the_text() {
        assert_eq!(bracket("  a b "), "[  a b ]");
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
    fn keeps_emoji() {
        assert_eq!(bracket("😀 東京"), "[😀 東京]");
    }
}
