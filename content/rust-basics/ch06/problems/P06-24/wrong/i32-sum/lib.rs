pub fn analyze(values: &[Option<i32>]) -> Option<(i32, i32, i64, usize)> {
    let mut result: Option<(i32, i32, i64, usize)> = None;
    for value in values {
        if let Some(n) = value {
            result = match result {
                None => Some((*n, *n, *n as i64, 1)),
                Some((lo, hi, sum, count)) => {
                    Some((lo.min(*n), hi.max(*n), (sum as i32 + *n) as i64, count + 1))
                }
            };
        }
    }
    result
}
