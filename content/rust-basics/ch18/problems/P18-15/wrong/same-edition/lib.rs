pub struct Post {
    content: String,
    edition: u32,
}

pub struct DraftPost {
    content: String,
    edition: u32,
}

pub struct PendingReviewPost {
    content: String,
    edition: u32,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
            edition: 1,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn edition(&self) -> u32 {
        self.edition
    }

    pub fn summary(&self) -> String {
        format!("edition {}: {}", self.edition, self.content)
    }

    pub fn next_edition(&self) -> DraftPost {
        DraftPost {
            content: self.content.clone(),
            edition: self.edition,
        }
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn replace_text(&mut self, text: &str) {
        self.content = text.to_string();
    }

    pub fn edition(&self) -> u32 {
        self.edition
    }

    pub fn char_count(&self) -> usize {
        self.content.chars().count()
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            edition: self.edition,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
            edition: self.edition,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
            edition: self.edition,
        }
    }
}

pub fn publish_all(drafts: Vec<DraftPost>) -> Vec<Post> {
    drafts
        .into_iter()
        .map(|d| d.request_review().approve())
        .collect()
}
