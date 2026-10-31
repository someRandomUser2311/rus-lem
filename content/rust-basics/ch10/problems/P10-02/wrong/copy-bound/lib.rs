pub fn minimum<T: PartialOrd + Copy>(values: &[T]) -> Option<&T> {
    if values.is_empty() {
        return None;
    }
    let mut best = &values[0];
    for value in values {
        if value < best {
            best = value;
        }
    }
    Some(best)
}
