#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub id: u32,
    pub label: String,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.label)
    }
}

impl From<(u32, String)> for Record {
    fn from(value: (u32, String)) -> Self {
        Self {
            id: value.0,
            label: value.1,
        }
    }
}

pub fn record_index(mut records: Vec<Record>) -> Vec<String> {
    records.sort();
    records.dedup();
    let mut out = Vec::new();
    for record in records {
        out.push(record.to_string());
    }
    out
}
