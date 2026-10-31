use std::collections::{HashMap, HashSet};

use crate::{
    book::{Book, Status},
    error::LibraryError,
    fees::{PREMIUM_LOAN_DAYS, due_day},
    member::Member,
    search::search,
    text::{sort_key, title_matches},
};

pub struct Library {
    books: Vec<Book>,
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

    pub fn add_member(&mut self, id: u32, name: &str, premium: bool) -> Result<(), LibraryError> {
        if self.members.contains_key(&id) {
            return Err(LibraryError::DuplicateMember(id));
        }
        self.members.insert(id, Member::new(id, name, premium));
        Ok(())
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

    pub fn checkout(
        &mut self,
        book_id: u32,
        member_id: u32,
        today: u32,
    ) -> Result<u32, LibraryError> {
        let i = self
            .index_of(book_id)
            .ok_or(LibraryError::NoSuchBook(book_id))?;
        let member = self
            .members
            .get_mut(&member_id)
            .ok_or(LibraryError::NoSuchMember(member_id))?;
        if !self.books[i].is_available() {
            return Err(LibraryError::NotAvailable(book_id));
        }
        if !member.can_borrow() {
            return Err(LibraryError::LimitReached(member_id));
        }
        if today > u32::MAX - PREMIUM_LOAN_DAYS {
            return Err(LibraryError::BadCommand(format!(
                "checkout {book_id} {member_id} {today}"
            )));
        }
        if !self.books[i].checkout(member, today) {
            return Err(LibraryError::NotAvailable(book_id));
        }
        Ok(due_day(today, member.premium))
    }

    pub fn return_book(&mut self, book_id: u32, today: u32) -> Result<u32, LibraryError> {
        let i = self
            .index_of(book_id)
            .ok_or(LibraryError::NoSuchBook(book_id))?;
        let member_id = self.books[i]
            .borrower()
            .ok_or(LibraryError::NotBorrowed(book_id))?;
        let member = self
            .members
            .get_mut(&member_id)
            .ok_or(LibraryError::NoSuchMember(member_id))?;
        self.books[i]
            .return_to_shelf(member, today)
            .ok_or(LibraryError::NotBorrowed(book_id))
    }

    pub fn members_sorted(&self) -> Vec<&Member> {
        let mut ids = Vec::new();
        for id in self.members.keys() {
            ids.push(*id);
        }
        ids.sort();
        ids.reverse();
        let mut out = Vec::new();
        for id in ids {
            if let Some(member) = self.members.get(&id) {
                out.push(member);
            }
        }
        out
    }

    pub fn find(&self, query: &str) -> Vec<&Book> {
        search(&self.books, query)
    }
}
