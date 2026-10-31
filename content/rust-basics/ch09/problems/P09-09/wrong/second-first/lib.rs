pub fn required(values: &[i32], index: usize) -> Result<i32, String> {
    values.get(index).copied().ok_or(format!("missing index {index}"))
}

pub fn pair_sum(values: &[i32], first: usize, second: usize) -> Result<i64, String> {
    let b = required(values, second)?;
    let a = required(values, first)?;
    Ok(a as i64 + b as i64)
}
