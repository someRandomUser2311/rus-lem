pub struct Document<'a> {
    text: &'a str,
    words: Vec<&'a str>,
}

impl<'a> Document<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut words = Vec::new();
        for word in text.split_whitespace() {
            words.push(word);
        }
        Self { text, words }
    }

    pub fn first(&self) -> Option<&str> {
        self.words.first().copied()
    }

    pub fn at(&self, index: usize) -> Option<&'a str> {
        let mut n = 0;
        for word in self.text.split_whitespace() {
            if n == index {
                return Some(word);
            }
            n += 1;
        }
        None
    }

    pub fn words(&self) -> &[&'a str] {
        &self.words
    }

    pub fn text(&self) -> &'a str {
        self.text
    }

    pub fn label(&self) -> &'static str {
        "document"
    }
}
