pub trait View {
    fn selected(&self) -> &str;
    fn message(&self) -> &str;

    fn describe(&self) -> String {
        format!("{} / {}", self.selected(), self.message())
    }
}

pub struct Report<'a> {
    selected: &'a str,
    message: String,
}

impl<'a> View for Report<'a> {
    fn selected(&self) -> &str {
        self.selected
    }

    fn message(&self) -> &str {
        &self.message
    }
}

pub fn borrowed_report<'a, T: std::fmt::Display>(
    a: &'a str,
    b: &'a str,
    announcement: &T,
) -> impl View + use<'a, T> {
    let selected = if a.len() >= b.len() { a } else { b };
    Report {
        selected,
        message: format!("Selected: {announcement}"),
    }
}

pub fn describe(view: &impl View) -> String {
    format!("{} / {}", view.selected(), view.message())
}
