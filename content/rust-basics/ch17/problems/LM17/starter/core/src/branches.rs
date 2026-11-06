use std::time::Duration;
use trpl::Either;

use crate::cli::load_catalog;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    pub name: String,
    pub delay_ms: u64,
    pub catalog: String,
}

pub fn branch(name: &str, delay_ms: u64, catalog: &str) -> Branch {
    Branch {
        name: name.to_string(),
        delay_ms,
        catalog: catalog.to_string(),
    }
}

pub async fn fetch_catalog(branch: &Branch) -> String {
    String::new()
}

pub async fn sync_branch(branch: Branch, limit_ms: u64) -> Result<usize, String> {
    Ok(0)
}

pub async fn sync_all(branches: Vec<Branch>, limit_ms: u64) -> Vec<Result<usize, String>> {
    Vec::new()
}

pub fn sync_report(branches: Vec<Branch>, limit_ms: u64) -> String {
    String::new()
}
