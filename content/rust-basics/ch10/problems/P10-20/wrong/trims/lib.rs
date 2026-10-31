pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Self { part }
    }

    pub fn text(&self) -> &str {
        self.part.trim()
    }

    pub fn original(&self) -> &'a str {
        self.part
    }

    pub fn kind(&self) -> &'static str {
        "excerpt"
    }
}
