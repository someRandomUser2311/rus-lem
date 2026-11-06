pub fn capped_sum(values: &[i32], cap: u64) -> u64 {
    values.iter().fold(0u64, |acc, n| {
        let amount = if *n > 0 { *n as u64 } else { 0 };
        (acc + amount).min(cap)
    })
}
