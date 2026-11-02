#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_a_word() {
        assert_eq!(bracket("rust"), "[rust]");
    }

    #[test]
    fn keeps_emoji() {
        assert_eq!(bracket("😀"), "[😀]");
    }
}
