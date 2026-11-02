#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_values_in_range() {
        assert_eq!(bounded(1), 1);
        assert_eq!(bounded(50), 50);
        assert_eq!(bounded(100), 100);
    }

    #[test]
    #[should_panic]
    fn zero_panics() {
        bounded(0);
    }

    #[test]
    #[should_panic]
    fn one_hundred_and_one_panics() {
        bounded(101);
    }
}
