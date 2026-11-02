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
}
