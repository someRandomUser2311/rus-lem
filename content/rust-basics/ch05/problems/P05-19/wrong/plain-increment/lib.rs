#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub title: String,
    pub body: String,
    pub revision: u32,
}
pub fn record(title: String, body: String, revision: u32) -> Record {
    Record {
        title,
        body,
        revision,
    }
}
pub fn revise(base: &Record, title: &str) -> Record {
    Record {
        title: title.to_string(),
        revision: base.revision + 1,
        ..base.clone()
    }
}
pub fn retitle(base: Record, title: String) -> Record {
    Record { title, ..base }
}
