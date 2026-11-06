pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    log: Vec<String>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
            log: Vec::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match &self.state {
            Some(state) => state.content(self),
            None => "",
        }
    }

    pub fn request_review(&mut self) {
        self.change(|s| s.request_review());
    }

    pub fn approve(&mut self) {
        self.change(|s| s.approve());
    }

    pub fn reject(&mut self) {
        self.change(|s| s.reject());
    }

    pub fn status(&self) -> &'static str {
        match &self.state {
            Some(state) => state.name(),
            None => "",
        }
    }

    pub fn transitions(&self) -> Vec<String> {
        self.log.clone()
    }

    fn change(&mut self, f: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
        if let Some(state) = self.state.take() {
            let before = state.name();
            let after = f(state);
            self.log.push(format!("{before} -> {}", after.name()));
            self.state = Some(after);
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn name(&self) -> &'static str;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;

struct PendingReview;

struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "draft"
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }

    fn name(&self) -> &'static str {
        "pending"
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "published"
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
