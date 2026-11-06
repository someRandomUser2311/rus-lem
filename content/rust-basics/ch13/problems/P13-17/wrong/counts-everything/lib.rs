pub fn first_above(values: &[i32], threshold: i32) -> (Option<usize>, usize) {
    let mut calls = 0;
    let index = values.iter().position(|n| {
        calls += 1;
        *n > threshold
    });
    (index, values.len())
}
