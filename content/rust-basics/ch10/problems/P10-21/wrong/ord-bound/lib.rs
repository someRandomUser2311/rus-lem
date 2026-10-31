pub fn extrema<T: Ord>(values: &[T]) -> Option<(&T, &T)> {
    if values.is_empty() {
        return None;
    }
    let mut low = &values[0];
    let mut high = &values[0];
    for value in values {
        if value < low {
            low = value;
        }
        if value > high {
            high = value;
        }
    }
    Some((low, high))
}
