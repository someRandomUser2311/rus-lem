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
        String::from("announcer")
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        match event {
            Event::Added { title, .. } => Some(format!("new book: {title}")),
            _ => None,
        }
    }
}

impl Plugin for LateFeeNotifier {
    fn name(&self) -> String {
        String::from("late-fees")
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        match event {
            Event::Returned { book_id, fee_cents } if *fee_cents > 0 => Some(format!(
                "book {book_id} returned late: {}",
                format_cents(*fee_cents)
            )),
            _ => None,
        }
    }
}

impl Plugin for BorrowCounter {
    fn name(&self) -> String {
        String::from("borrows")
    }

    fn on_event(&mut self, event: &Event) -> Option<String> {
        if let Event::CheckedOut { .. } = event {
            self.count = self.count.saturating_add(1);
        }
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

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name()).collect()
    }

    pub fn library(&self) -> &Library {
        &self.library
    }

    pub fn add_book(&mut self, title: &str, author: &str, year: u16) -> (u32, Vec<String>) {
        let book_id = self.library.add_book(title, author, year);
        let sent = self.emit(Event::Added {
            book_id,
            title: title.to_string(),
        });
        (book_id, sent)
    }

    pub fn checkout(
        &mut self,
        book_id: u32,
        member_id: u32,
        today: u32,
    ) -> Result<(u32, Vec<String>), LibraryError> {
        let due = self.library.checkout(book_id, member_id, today)?;
        let sent = self.emit(Event::CheckedOut { book_id, member_id });
        Ok((due, sent))
    }

    pub fn return_book(&mut self, book_id: u32, today: u32) -> Result<(u32, Vec<String>), LibraryError> {
        let fee_cents = self.library.return_book(book_id, today)?;
        let sent = self.emit(Event::Returned { book_id, fee_cents });
        Ok((fee_cents, sent))
    }

    fn emit(&mut self, event: Event) -> Vec<String> {
        let mut sent = Vec::new();
        for plugin in self.plugins.iter_mut() {
            let name = plugin.name();
            if let Some(message) = plugin.on_event(&event) {
                sent.push(format!("[{name}] {message}"));
            }
        }
        sent
    }
}
