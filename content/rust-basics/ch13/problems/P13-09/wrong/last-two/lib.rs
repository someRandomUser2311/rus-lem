pub fn first_two(values: &[i32]) -> (Option<i32>, Option<i32>) {
    let mut it = values.iter().rev();
    let a = it.next().copied();
    let b = it.next().copied();
    (a, b)
}
