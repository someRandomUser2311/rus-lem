use crate::error::LibraryError;
use crate::fees::format_cents;
use crate::library::Library;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Added { book_id: u32, title: String },
    CheckedOut { book_id: u32, member_id: u32 },
    Returned { book_id: u32, fee_cents: u32 },
}

pub trait Plugin: Send {
    fn name(&self) -> String;
    fn on_event(&mut self, event: &Event) -> Option<String>;
}

pub struct Announcer;

pub struct LateFeeNotifier;

pub struct BorrowCounter {
    count: u32,
}

impl BorrowCounter {
    pub fn new() -> BorrowCounter {
        BorrowCounter { count: 0 }
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

impl Plugin for Announcer {
    fn name(&self) -> String {
        String::new()
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        None
    }
}

impl Plugin for LateFeeNotifier {
    fn name(&self) -> String {
        String::new()
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        None
    }
}

impl Plugin for BorrowCounter {
    fn name(&self) -> String {
        String::new()
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        None
    }
}

pub struct Notifier {
    library: Library,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Notifier {
    pub fn new(library: Library) -> Notifier {
        Notifier {
            library,
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {}

    pub fn plugin_names(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn add_book(&mut self, title: &str, author: &str, year: u16) -> (u32, Vec<String>) {
        (0, Vec::new())
    }

    pub fn checkout(
        &mut self,
        book_id: u32,
        member_id: u32,
        today: u32,
    ) -> Result<(u32, Vec<String>), LibraryError> {
        Ok((0, Vec::new()))
    }

    pub fn return_book(&mut self, book_id: u32, today: u32) -> Result<(u32, Vec<String>), LibraryError> {
        Ok((0, Vec::new()))
    }
}
