pub fn first<T: Copy>(values: &[T]) -> Option<&T> {
    if values.is_empty() {
        None
    } else {
        Some(&values[0])
    }
}
