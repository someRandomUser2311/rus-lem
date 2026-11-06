fn record(line: &str, n: usize) -> Result<(String, u32), String> {
    let error = || format!("line {n}: expected label|count");
    let mut fields = line.split('|');
    let label = fields.next().ok_or_else(error)?.trim();
    let count = fields.next().ok_or_else(error)?.trim();
    if fields.next().is_some() || label.is_empty() {
        return Err(error());
    }
    let value = count.parse::<u32>().map_err(|_| error())?;
    Ok((label.to_string(), value))
}

pub fn parse_records(lines: &[String]) -> Result<Vec<(String, u32)>, String> {
    lines
        .iter()
        .filter(|line| {
            let s = line.trim();
            !s.is_empty() && !s.starts_with('#')
        })
        .enumerate()
        .map(|(i, line)| record(line.trim(), i + 1))
        .collect()
}
