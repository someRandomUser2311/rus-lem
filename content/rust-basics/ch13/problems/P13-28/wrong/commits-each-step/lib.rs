pub fn transact<F: FnMut(i64) -> Result<i64, String>>(
    balance: &mut i64,
    deltas: &[i64],
    mut validate: F,
) -> Result<Vec<i64>, String> {
    let mut current = *balance;
    let result = deltas
        .iter()
        .map(|delta| {
            let candidate = current
                .checked_add(*delta)
                .ok_or_else(|| "overflow".to_string())?;
            current = validate(candidate)?;
            *balance = current;
            Ok(current)
        })
        .collect::<Result<Vec<i64>, String>>()?;
    *balance = current;
    Ok(result)
}
