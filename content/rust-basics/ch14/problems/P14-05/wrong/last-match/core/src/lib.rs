//! Searching and printing catalog records.

mod query;

pub use catalog_models::Record;
pub use query::{first_named, render};
