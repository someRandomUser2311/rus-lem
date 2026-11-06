use std::sync::mpsc;
use std::thread;

use crate::Library;
use crate::cli::load_catalog;

pub fn library_is_shareable() -> bool {
    false
}

pub fn search_many(library: &Library, queries: Vec<String>, workers: usize) -> Vec<Vec<u32>> {
    Vec::new()
}

pub fn load_many(catalogs: Vec<String>) -> Vec<Result<usize, String>> {
    Vec::new()
}
