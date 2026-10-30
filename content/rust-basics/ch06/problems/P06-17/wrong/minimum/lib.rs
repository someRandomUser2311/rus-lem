pub fn maximum(values: &[Option<i32>]) -> Option<i32> {
    let mut best = None;
    for value in values {
        if let Some(n) = value {
            match best {
                None => best = Some(*n),
                Some(old) => {
                    if *n < old {
                        best = Some(*n);
                    }
                }
            }
        }
    }
    best
}
