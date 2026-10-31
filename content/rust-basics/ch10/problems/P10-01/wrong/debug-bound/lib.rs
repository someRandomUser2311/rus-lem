pub fn first<T: std::fmt::Debug>(values: &[T]) -> Option<&T> {
    if values.is_empty() {
        None
    } else {
        Some(&values[0])
    }
}
