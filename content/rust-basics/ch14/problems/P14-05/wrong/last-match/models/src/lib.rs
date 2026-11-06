//! Records in the catalog.

/// One catalog entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub id: u32,
    pub name: String,
}
