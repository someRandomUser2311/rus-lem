use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    Shape(usize),
    EmptyKey(usize),
    InvalidNumber(usize),
    Duplicate(String),
}

pub fn parse_line(line: &str, number: usize) -> Result<(String, u32), ConfigError> {
    let mut parts = line.split('=');
    let key = parts.next().ok_or(ConfigError::Shape(number))?.trim();
    let value = parts.next().ok_or(ConfigError::Shape(number))?.trim();
    if parts.next().is_some() {
        return Err(ConfigError::Shape(number));
    }
    if key.is_empty() {
        return Err(ConfigError::EmptyKey(number));
    }
    let value = value
        .parse::<u32>()
        .ok()
        .ok_or(ConfigError::InvalidNumber(number))?;
    Ok((key.to_string(), value))
}

pub fn settings(text: &str) -> Result<Vec<(String, u32)>, ConfigError> {
    let mut seen: HashMap<String, u32> = HashMap::new();
    let mut number = 0;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        number += 1;
        let (key, value) = parse_line(line, number)?;
        if seen.contains_key(&key) {
            return Err(ConfigError::Duplicate(key));
        }
        seen.insert(key, value);
    }
    let mut pairs = Vec::new();
    for (key, value) in seen {
        pairs.push((key, value));
    }
    pairs.sort();
    Ok(pairs)
}
