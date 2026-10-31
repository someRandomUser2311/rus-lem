pub fn extrema<T: PartialOrd>(values: &[T]) -> Option<(&T, &T)> {
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
    Some((high, low))
}
