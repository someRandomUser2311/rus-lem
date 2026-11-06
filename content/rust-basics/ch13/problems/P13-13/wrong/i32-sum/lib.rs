pub fn totals(values: &[i32]) -> (i64, usize) {
    let sum = values.iter().sum::<i32>() as i64;
    let count = values.iter().count();
    (sum, count)
}
