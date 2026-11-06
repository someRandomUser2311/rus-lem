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
    trpl::sleep(Duration::from_millis(branch.delay_ms)).await;
    branch.catalog.clone()
}

pub async fn sync_branch(branch: Branch, limit_ms: u64) -> Result<usize, String> {
    let limit = trpl::sleep(Duration::from_millis(limit_ms));
    match trpl::select(fetch_catalog(&branch), limit).await {
        Either::Left(text) => load_catalog(&text)
            .map(|lib| lib.find("").len())
            .map_err(|e| format!("{}: {e}", branch.name)),
        Either::Right(()) => Err(format!("{}: timed out", branch.name)),
    }
}

pub async fn sync_all(branches: Vec<Branch>, limit_ms: u64) -> Vec<Result<usize, String>> {
    trpl::join_all(branches.into_iter().map(|b| sync_branch(b, limit_ms))).await
}

pub fn sync_report(branches: Vec<Branch>, limit_ms: u64) -> String {
    let names: Vec<String> = branches.iter().map(|b| b.name.clone()).collect();
    let results = trpl::block_on(sync_all(branches, limit_ms));
    let mut out = String::new();
    let (mut synced, mut books) = (0, 0);
    for (name, result) in names.iter().zip(&results) {
        match result {
            Ok(n) => {
                synced += 1;
                books += n;
                let unit = if *n == 1 { "book" } else { "books" };
                out.push_str(&format!("{name}: {n} {unit}\n"));
            }
            Err(e) => out.push_str(&format!("{e}\n")),
        }
    }
    out.push_str(&format!(
        "synced {synced} of {} branches, {books} books\n",
        results.len()
    ));
    out
}
