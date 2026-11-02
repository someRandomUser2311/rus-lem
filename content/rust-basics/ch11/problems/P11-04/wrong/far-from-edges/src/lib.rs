#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_in_range_values() {
        assert_eq!(clamp_byte(0), 0, "0 stays 0");
        assert_eq!(clamp_byte(100), 100, "100 stays 100");
        assert_eq!(clamp_byte(255), 255, "255 stays 255");
    }

    #[test]
    fn clamps_negatives_to_zero() {
        assert_eq!(clamp_byte(-5), 0, "-5 clamps to 0");
        assert_ne!(clamp_byte(-5), 251, "-5 must not wrap around");
    }

    #[test]
    fn clamps_large_values_to_255() {
        assert_eq!(clamp_byte(300), 255, "300 clamps to 255");
    }
}
