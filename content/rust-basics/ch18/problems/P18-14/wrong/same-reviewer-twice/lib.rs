pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match &self.state {
            Some(state) if state.published() => &self.content,
            _ => "",
        }
    }

    pub fn status(&self) -> String {
        match &self.state {
            Some(state) => state.status(),
            None => String::new(),
        }
    }

    pub fn approvers(&self) -> Vec<u32> {
        match &self.state {
            Some(state) => state.approvers(),
            None => Vec::new(),
        }
    }

    pub fn request_review(&mut self) {
        self.change(|s| s.request_review());
    }

    pub fn approve(&mut self, reviewer: u32) {
        self.change(|s| s.approve(reviewer));
    }

    pub fn reject(&mut self) {
        self.change(|s| s.reject());
    }

    fn change(&mut self, f: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
        if let Some(state) = self.state.take() {
            self.state = Some(f(state));
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, reviewer: u32) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn status(&self) -> String;
    fn approvers(&self) -> Vec<u32> {
        Vec::new()
    }
    fn published(&self) -> bool {
        false
    }
}

struct Draft;

struct PendingReview {
    approvals: Vec<u32>,
}

struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approvals: Vec::new(),
        })
    }

    fn approve(self: Box<Self>, _reviewer: u32) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn status(&self) -> String {
        String::from("draft")
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>, reviewer: u32) -> Box<dyn State> {
        self.approvals.push(reviewer);
        if self.approvals.len() >= 2 {
            Box::new(Published)
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }

    fn status(&self) -> String {
        format!("pending ({}/2)", self.approvals.len())
    }

    fn approvers(&self) -> Vec<u32> {
        self.approvals.clone()
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, _reviewer: u32) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn status(&self) -> String {
        String::from("published")
    }

    fn published(&self) -> bool {
        true
    }
}
