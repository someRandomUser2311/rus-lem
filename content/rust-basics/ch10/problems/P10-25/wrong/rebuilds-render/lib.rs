pub trait PageReport {
    fn title(&self) -> String;
    fn lines(&self) -> Vec<String>;

    fn render(&self) -> String {
        format!("{}\n{}", self.title(), self.lines().join("\n"))
    }
}

pub struct Page {
    pub title: String,
    pub body: Vec<String>,
}

impl PageReport for Page {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn lines(&self) -> Vec<String> {
        self.body.clone()
    }
}

pub fn render_pages<T: PageReport>(pages: &[T]) -> String {
    let mut parts = Vec::new();
    for page in pages {
        parts.push(format!("{}\n{}", page.title(), page.lines().join("\n")));
    }
    parts.join("\n---\n")
}
