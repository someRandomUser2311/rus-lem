#[derive(Debug, Clone, PartialEq)]
pub struct Draft {
    pub title: String,
    pub text: String,
    pub edition: u32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Published {
    pub title: String,
    pub text: String,
    pub edition: u32,
}
impl Draft {
    pub fn new(title: &str, text: &str) -> Self {
        Self {
            title: title.to_string(),
            text: text.to_string(),
            edition: 1,
        }
    }
    pub fn fork(&self, title: &str) -> Self {
        Self {
            title: title.trim().to_string(),
            edition: self.edition.saturating_add(1),
            ..self.clone()
        }
    }
    pub fn append(&mut self, text: &str) {
        self.text.push_str(text);
    }
    pub fn publish(self) -> Published {
        Published {
            title: self.title,
            text: self.text,
            edition: self.edition,
        }
    }
}
impl Published {
    pub fn summary(&self) -> String {
        format!(
            "{} [edition {}]: {} bytes",
            self.title,
            self.edition,
            self.text.len()
        )
    }
    pub fn into_draft(self) -> Draft {
        Draft {
            title: self.title,
            text: self.text,
            edition: self.edition,
        }
    }
}
