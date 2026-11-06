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
        write!(f, "")
    }
}

impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl From<u32> for BookId {
    fn from(n: u32) -> BookId {
        BookId(0)
    }
}

impl From<u32> for MemberId {
    fn from(n: u32) -> MemberId {
        MemberId(0)
    }
}

impl Add for Cents {
    type Output = Cents;
    fn add(self, other: Cents) -> Cents {
        self
    }
}

impl AddAssign for Cents {
    fn add_assign(&mut self, other: Cents) {}
}

impl Sum for Cents {
    fn sum<I: Iterator<Item = Cents>>(iter: I) -> Cents {
        Cents(0)
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub fn checkout(lib: &mut Library, book: BookId, member: MemberId, today: u32) -> LibResult<u32> {
    Ok(0)
}

pub fn return_book(lib: &mut Library, book: BookId, today: u32) -> LibResult<Cents> {
    Ok(Cents(0))
}

#[macro_export]
macro_rules! catalogue {
    () => {
        $crate::Library::new()
    };
}
