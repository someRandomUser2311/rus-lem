#[derive(Debug, Clone, PartialEq)]
pub enum LibraryError {
    NoSuchBook(u32),
    NoSuchMember(u32),
    NotAvailable(u32),
    LimitReached(u32),
    NotBorrowed(u32),
    DuplicateMember(u32),
    BadCommand(String),
}
