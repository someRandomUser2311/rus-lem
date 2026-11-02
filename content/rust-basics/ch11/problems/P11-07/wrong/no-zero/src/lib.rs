#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_plain_number() {
        assert_eq!(count("7"), Ok(7));
    }

    #[test]
    fn reads_padded_numbers_and_zero() -> Result<(), String> {
        let n = count(" 42 ")?;
        assert_eq!(n, 42);
        Ok(())
    }

    #[test]
    fn rejects_text_that_is_not_a_count() {
        assert_eq!(count("abc"), Err(String::from("invalid")));
        assert_eq!(count("-3"), Err(String::from("invalid")));
        assert_eq!(count(""), Err(String::from("invalid")));
    }
}
