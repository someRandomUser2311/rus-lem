//! The data types shared by the score packages.

/// A labelled number of points.
#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub label: String,
    pub points: i32,
}
