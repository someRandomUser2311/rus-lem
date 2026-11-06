pub fn positive_count(text: Option<&str>) -> Result<u32, String> {
    let s = text.ok_or_else(|| "missing".to_string())?;
    let s = s.trim();
    s.parse::<u32>()
        .map_err(|_| format!("invalid: {s}"))
        .and_then(|n| {
            if n == 0 {
                Err("must be positive".to_string())
            } else {
                Ok(n)
            }
        })
}
