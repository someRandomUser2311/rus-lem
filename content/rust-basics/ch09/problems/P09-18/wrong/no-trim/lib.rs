pub fn optional_parse(text: Option<&str>, cap: u32) -> Option<u32> {
    let text = text?;
    let n = text.parse::<u32>().ok()?;
    if n <= cap { Some(n) } else { None }
}

pub fn capped_total(fields: &[Option<&str>], cap: u32) -> Option<u32> {
    let mut total: u32 = 0;
    for field in fields {
        let n = optional_parse(*field, cap)?;
        total = total.checked_add(n)?;
    }
    Some(total)
}
