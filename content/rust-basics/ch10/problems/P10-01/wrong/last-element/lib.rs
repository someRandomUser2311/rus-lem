pub fn first<T>(values: &[T]) -> Option<&T> {
    if values.is_empty() {
        None
    } else {
        Some(&values[values.len() - 1])
    }
}
