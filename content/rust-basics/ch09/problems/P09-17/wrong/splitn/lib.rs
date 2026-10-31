#[derive(Debug, Clone, PartialEq)]
pub enum RecordError {
    Fields,
    Name,
    Age,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub age: u8,
}

pub fn record(text: &str) -> Result<Record, RecordError> {
    let mut parts = text.splitn(2, '|');
    let name = parts.next().ok_or(RecordError::Fields)?.trim();
    let age = parts.next().ok_or(RecordError::Fields)?.trim();
    if parts.next().is_some() {
        return Err(RecordError::Fields);
    }
    if name.is_empty() {
        return Err(RecordError::Name);
    }
    let age = age.parse::<u8>().ok().ok_or(RecordError::Age)?;
    Ok(Record {
        name: name.to_string(),
        age,
    })
}
