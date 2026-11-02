#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cheap_small() {
        assert_eq!(triangular(3), 6);
    }

    #[test]
    fn cheap_zero() {
        assert_eq!(triangular(0), 0);
    }

    #[test]
    fn ignored_boundary() {
        assert_eq!(triangular(u32::MAX), 9_223_372_034_707_292_160);
    }
}
