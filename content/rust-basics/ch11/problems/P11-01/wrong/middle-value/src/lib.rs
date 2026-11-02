#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_two() {
        assert_eq!(double(2), 4);
    }

    #[test]
    fn doubles_zero() {
        assert_eq!(double(0), 0);
    }

    #[test]
    fn doubles_three() {
        assert_eq!(double(3), 6);
    }
}
