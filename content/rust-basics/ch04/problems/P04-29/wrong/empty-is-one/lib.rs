pub fn longest_run(values: &[i32]) -> (usize, usize) {
    if values.is_empty() {
        return (0, 1);
    }
    let mut best_start = 0;
    let mut best_len = 1;
    let mut start = 0;
    for i in 1..values.len() {
        if values[i] != values[i - 1] {
            start = i;
        }
        let length = i - start + 1;
        if length > best_len {
            best_start = start;
            best_len = length;
        }
    }
    (best_start, best_len)
}
