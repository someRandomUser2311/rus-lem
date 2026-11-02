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
}
