use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign};

use crate::error::LibraryError;
use crate::fees::format_cents;
use crate::library::Library;

pub type LibResult<T> = Result<T, LibraryError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BookId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemberId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Cents(pub u32);

impl fmt::Display for BookId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for BookId {
    fn from(n: u32) -> BookId {
        BookId(n)
    }
}

impl From<u32> for MemberId {
    fn from(n: u32) -> MemberId {
        MemberId(n)
    }
}

impl Add for Cents {
    type Output = Cents;
    fn add(self, other: Cents) -> Cents {
        Cents(self.0.saturating_add(other.0))
    }
}

impl AddAssign for Cents {
    fn add_assign(&mut self, other: Cents) {
        *self = *self + other;
    }
}

impl Sum for Cents {
    fn sum<I: Iterator<Item = Cents>>(iter: I) -> Cents {
        iter.fold(Cents(0), |a, b| a + b)
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_cents(self.0))
    }
}

pub fn checkout(lib: &mut Library, book: BookId, member: MemberId, today: u32) -> LibResult<u32> {
    lib.checkout(book.0, member.0, today)
}

pub fn return_book(lib: &mut Library, book: BookId, today: u32) -> LibResult<Cents> {
    lib.return_book(book.0, today).map(Cents)
}

#[macro_export]
macro_rules! catalogue {
    ($($title:literal by $author:literal ($year:expr)),* $(,)?) => {{
        let mut lib = $crate::Library::new();
        $(lib.add_book($title, $author, $year);)*
        lib
    }};
}
