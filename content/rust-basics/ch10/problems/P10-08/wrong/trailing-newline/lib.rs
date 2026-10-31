pub trait Report {
    fn report(&self) -> String;
}

pub struct News {
    pub title: String,
    pub author: String,
}

pub struct Alert {
    pub code: u32,
}

impl Report for News {
    fn report(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Report for Alert {
    fn report(&self) -> String {
        format!("alert {}\n", self.code)
    }
}
