/// Reads one amount, ignoring spaces around it.
///
/// # Examples
///
/// ```
/// use amounts_core::parse_amount;
///
/// assert_eq!(parse_amount(" +7 "), Ok(7));
/// assert_eq!(parse_amount("x"), Err(String::from("invalid amount: x")));
/// ```
pub fn parse_amount(text: &str) -> Result<u32, String> {
    text.trim()
        .parse::<u32>()
        .map_err(|_| format!("invalid amount: {text}"))
}

/// Adds up every amount, or returns the first error.
///
/// # Examples
///
/// ```
/// use amounts_core::total;
///
/// assert_eq!(total(&["1", " 2 "]), Ok(3));
/// ```
pub fn total(texts: &[&str]) -> Result<u64, String> {
    let mut sum: u32 = 0;
    for t in texts {
        sum += parse_amount(t)?;
    }
    Ok(sum as u64)
}
