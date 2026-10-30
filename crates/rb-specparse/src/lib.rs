//! Reads the course plan (`rust-basics-course-plan/`) into the content model.
//!
//! The plan is the reviewed spec; this crate produces the generated layer that
//! `course-check import` writes. Anything it can't convert is left as `Todo` for
//! the authored overlay.

pub mod course;
pub mod lessons;
pub mod md;
pub mod problems;
pub mod questions;

/// Problems found while parsing.
#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Report {
    pub fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}
