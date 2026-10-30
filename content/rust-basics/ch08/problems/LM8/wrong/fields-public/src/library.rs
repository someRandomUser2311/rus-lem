use std::collections::{HashMap, HashSet};

use crate::{
    book::{Book, Status},
    member::Member,
    text::{sort_key, title_matches},
};

pub struct Library {
    pub books: Vec<Book>,
    members: HashMap<u32, Member>,
    next_book_id: u32,
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
            members: HashMap::new(),
            next_book_id: 1,
        }
    }

    fn index_of(&self, id: u32) -> Option<usize> {
        for (i, book) in self.books.iter().enumerate() {
            if book.id == id {
                return Some(i);
            }
        }
        None
    }

    pub fn add_book(&mut self, title: &str, author: &str, year: u16) -> u32 {
        let id = self.next_book_id;
        self.next_book_id += 1;
        self.books.push(Book::new(id, title, author, year));
        id
    }

    pub fn add_member(&mut self, id: u32, name: &str, premium: bool) -> bool {
        if self.members.contains_key(&id) {
            return false;
        }
        self.members.insert(id, Member::new(id, name, premium));
        true
    }

    pub fn book(&self, id: u32) -> Option<&Book> {
        for book in &self.books {
            if book.id == id {
                return Some(book);
            }
        }
        None
    }

    pub fn member(&self, id: u32) -> Option<&Member> {
        self.members.get(&id)
    }

    pub fn books_by(&self, author: &str) -> Vec<&Book> {
        let wanted = author.trim().to_lowercase();
        let mut keys = Vec::new();
        for (i, book) in self.books.iter().enumerate() {
            if book.author.trim().to_lowercase() == wanted {
                keys.push((sort_key(&book.title), book.id, i));
            }
        }
        keys.sort();
        let mut out = Vec::new();
        for (_, _, i) in keys {
            out.push(&self.books[i]);
        }
        out
    }

    pub fn search_titles(&self, query: &str) -> Vec<u32> {
        let mut ids = Vec::new();
        for book in &self.books {
            if title_matches(&book.title, query) {
                ids.push(book.id);
            }
        }
        ids
    }

    pub fn count_by_status(&self) -> (usize, usize, usize) {
        let mut available = 0;
        let mut borrowed = 0;
        let mut lost = 0;
        for book in &self.books {
            match book.status {
                Status::Available => available += 1,
                Status::Borrowed {
                    member_id: _,
                    due_day: _,
                } => borrowed += 1,
                Status::Lost => lost += 1,
            }
        }
        (available, borrowed, lost)
    }

    pub fn authors(&self) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for book in &self.books {
            if seen.insert(book.author.as_str()) {
                out.push(book.author.clone());
            }
        }
        out.sort();
        out
    }

    pub fn checkout(&mut self, book_id: u32, member_id: u32, today: u32) -> bool {
        let Some(i) = self.index_of(book_id) else {
            return false;
        };
        let Some(member) = self.members.get_mut(&member_id) else {
            return false;
        };
        self.books[i].checkout(member, today)
    }

    pub fn return_book(&mut self, book_id: u32, today: u32) -> Option<u32> {
        let Some(i) = self.index_of(book_id) else {
            return None;
        };
        let Some(member_id) = self.books[i].borrower() else {
            return None;
        };
        let Some(member) = self.members.get_mut(&member_id) else {
            return None;
        };
        self.books[i].return_to_shelf(member, today)
    }
}
