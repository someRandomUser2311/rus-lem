pub fn maximum(values: &[Option<i32>]) -> Option<i32> {
    let mut best = i32::MIN;
    let mut seen = false;
    for value in values {
        if let Some(n) = value {
            seen = true;
            if *n > best {
                best = *n;
            }
        }
    }
    if seen { Some(best) } else { None }
}
