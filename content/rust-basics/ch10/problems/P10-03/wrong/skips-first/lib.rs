pub fn count_equal<T: PartialEq>(values: &[T], target: &T) -> usize {
    let mut count = 0;
    for value in &values[1.min(values.len())..] {
        if value == target {
            count += 1;
        }
    }
    count
}
