use crate::{book::Book, member::Member};

pub trait Searchable {
    fn search_text(&self) -> String;

    fn matches(&self, query: &str) -> bool {
        let text = self.search_text().to_lowercase();
        let query = query.to_lowercase();
        for word in query.split_whitespace() {
            if !text.contains(word) {
                return false;
            }
        }
        true
    }
}

impl Searchable for Book {
    fn search_text(&self) -> String {
        format!("{}{}", self.title, self.author)
    }
}

impl Searchable for Member {
    fn search_text(&self) -> String {
        self.name.clone()
    }
}

impl<T: Searchable> Searchable for &T {
    fn search_text(&self) -> String {
        (**self).search_text()
    }

    fn matches(&self, query: &str) -> bool {
        (**self).matches(query)
    }
}

pub fn search<'a, T: Searchable>(items: &'a [T], query: &str) -> Vec<&'a T> {
    let mut out = Vec::new();
    for item in items {
        if item.matches(query) {
            out.push(item);
        }
    }
    out
}
