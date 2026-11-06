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
            Some(state) => state.content(self),
            None => "",
        }
    }

    pub fn status(&self) -> &'static str {
        match &self.state {
            Some(state) => state.name(),
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

    fn change(&mut self, f: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
        if let Some(state) = self.state.take() {
            self.state = Some(f(state));
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

pub mod typed {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }
}

pub fn run(script: &str) -> String {
    let mut post = Post::new();
    let mut out = String::new();
    for (i, line) in script.lines().enumerate() {
        let Some(word) = line.split_whitespace().next() else {
            continue;
        };
        if let Some(text) = line.strip_prefix("add ") {
            post.add_text(text);
            continue;
        }
        match line.trim() {
            "review" => post.request_review(),
            "approve" => post.approve(),
            "reject" => post.reject(),
            "status" => out.push_str(&format!("status: {}\n", post.status())),
            "show" => out.push_str(&format!("content: {}\n", post.content())),
            _ => out.push_str(&format!("line {}: unknown command {word}\n", i + 1)),
        }
    }
    out
}
