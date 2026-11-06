//! A small library manager: books, members, loans and late fees.
//!
//! # Examples
//!
//! ```
//! use library::Library;
//!
//! let mut lib = Library::new();
//! lib.add_book("Dune", "Frank Herbert", 1965);
//! lib.add_book("Emma", "Jane Austen", 1815);
//! let found = lib.find("emma");
//! assert_eq!(found.len(), 1);
//! assert_eq!(found[0].title, "Emma");
//! ```

pub mod book;
pub mod cli;
pub mod command;
pub mod concurrent;
pub mod error;
pub mod fees;
pub mod households;
pub mod library;
pub mod member;
pub mod search;
pub mod text;

pub use book::{Book, Status};
pub use command::{Command, parse_command};
pub use error::LibraryError;
pub use library::Library;
pub use member::Member;
pub use search::{Searchable, search};
