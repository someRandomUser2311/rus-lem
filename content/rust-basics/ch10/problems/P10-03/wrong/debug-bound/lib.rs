pub fn count_equal<T: PartialEq + std::fmt::Debug>(values: &[T], target: &T) -> usize {
    let mut count = 0;
    for value in values {
        if value == target {
            count += 1;
        }
    }
    count
}
