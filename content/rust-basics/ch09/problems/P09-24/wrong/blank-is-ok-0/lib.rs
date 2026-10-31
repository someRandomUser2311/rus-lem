#[derive(Debug, Clone, PartialEq)]
pub enum RowError {
    Missing,
    Invalid(String),
    Overflow,
}

pub fn row_total(line: &str) -> Result<u32, RowError> {
    let mut total: u32 = 0;
    let mut seen = false;
    for token in line.split_whitespace() {
        seen = true;
        let n = match token.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(RowError::Invalid(token.to_string())),
        };
        total = total.checked_add(n).ok_or(RowError::Overflow)?;
    }
    Ok(total)
}

pub fn row_totals(text: &str) -> Vec<Result<u32, RowError>> {
    let mut out = Vec::new();
    for line in text.lines() {
        out.push(row_total(line));
    }
    out
}
