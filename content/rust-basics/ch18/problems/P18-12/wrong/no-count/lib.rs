pub struct Post {
    content: String,
    revisions: u32,
}

pub struct DraftPost {
    content: String,
    revisions: u32,
}

pub struct PendingReviewPost {
    content: String,
    revisions: u32,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
            revisions: 0,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn revisions(&self) -> u32 {
        self.revisions
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn replace_text(&mut self, text: &str) {
        self.content = text.to_string();
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            revisions: self.revisions,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
            revisions: self.revisions,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
            revisions: self.revisions,
        }
    }
}
