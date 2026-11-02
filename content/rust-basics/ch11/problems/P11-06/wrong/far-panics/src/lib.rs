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
    #[should_panic(expected = "below minimum")]
    fn minus_five_is_below_minimum() {
        bounded(-5);
    }

    #[test]
    #[should_panic(expected = "above maximum")]
    fn five_hundred_is_above_maximum() {
        bounded(500);
    }
}
