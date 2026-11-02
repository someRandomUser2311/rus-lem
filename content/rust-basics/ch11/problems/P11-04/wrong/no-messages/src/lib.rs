#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_in_range_values() {
        assert_eq!(clamp_byte(0), 0);
        assert_eq!(clamp_byte(100), 100);
        assert_eq!(clamp_byte(255), 255);
    }

    #[test]
    fn clamps_negatives_to_zero() {
        assert_eq!(clamp_byte(-1), 0);
        assert_eq!(clamp_byte(i16::MIN), 0);
        assert_ne!(clamp_byte(-1), 255);
    }

    #[test]
    fn clamps_large_values_to_255() {
        assert_eq!(clamp_byte(256), 255);
        assert_eq!(clamp_byte(i16::MAX), 255);
    }
}
