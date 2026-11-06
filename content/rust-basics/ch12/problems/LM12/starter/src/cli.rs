use crate::Library;

#[derive(Debug, PartialEq)]
pub enum Action {
    List,
    Count,
    Find(String),
    By(String),
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub catalog_path: String,
    pub action: Action,
    pub by_year: bool,
}

impl Config {
    pub fn build(args: &[String], sort: Option<String>) -> Result<Config, String> {
        Err(String::from("usage: library <catalog> <list|count|find|by> [words...]"))
    }
}

pub fn load_catalog(text: &str) -> Result<Library, String> {
    Ok(Library::new())
}

pub fn run(config: &Config, catalog: &str) -> Result<String, String> {
    Ok(String::new())
}
