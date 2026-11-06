use std::collections::HashMap;

use crate::text::sort_key;
use crate::{Book, Library};

#[derive(Debug, PartialEq)]
pub enum Action {
    List,
    Count,
    Find(String),
    By(String),
    Authors,
    Decades,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub catalog_path: String,
    pub action: Action,
    pub by_year: bool,
}

fn usage() -> String {
    String::from("usage: library <catalog> <list|count|find|by|authors|decades> [words...]")
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>, sort: Option<String>) -> Result<Config, String> {
        args.next();
        let catalog_path = args.next().ok_or_else(usage)?;
        let command = args.next().ok_or_else(usage)?;
        let words: Vec<String> = args.collect();
        let action = match command.as_str() {
            "list" | "count" | "authors" | "decades" => {
                if !words.is_empty() {
                    return Err(format!("{command} takes no words"));
                }
                match command.as_str() {
                    "list" => Action::List,
                    "count" => Action::Count,
                    "authors" => Action::Authors,
                    _ => Action::Decades,
                }
            }
            "find" | "by" => {
                if words.is_empty() {
                    return Err(format!("{command} needs at least one word"));
                }
                let joined = words.join(" ");
                if command == "find" {
                    Action::Find(joined)
                } else {
                    Action::By(joined)
                }
            }
            _ => return Err(format!("unknown command: {command}")),
        };
        Ok(Config {
            catalog_path,
            action,
            by_year: sort.as_deref() == Some("year"),
        })
    }
}

fn record(line: &str, n: usize) -> Result<(&str, &str, u16), String> {
    let error = || format!("line {n}: expected title|author|year");
    let fields: Vec<&str> = line.split('|').collect();
    if fields.len() != 3 {
        return Err(error());
    }
    let year = fields[2].trim().parse::<u16>().map_err(|_| error())?;
    Ok((fields[0], fields[1], year))
}

pub fn load_catalog(text: &str) -> Result<Library, String> {
    let records = text
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                None
            } else {
                Some(record(line, i + 1))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut library = Library::new();
    records.iter().for_each(|(title, author, year)| {
        library.add_book(title, author, *year);
    });
    Ok(library)
}

fn lines(books: &[&Book]) -> String {
    books
        .iter()
        .map(|b| format!("{}: {}\n", b.id, b.summary()))
        .collect()
}

pub fn run(config: &Config, catalog: &str) -> Result<String, String> {
    let library = load_catalog(catalog)?;
    let out = match &config.action {
        Action::Count => {
            let (available, borrowed, lost) = library.count_by_status();
            format!("Available: {available}\nBorrowed: {borrowed}\nLost: {lost}\n")
        }
        Action::Find(query) => lines(&library.find(query)),
        Action::By(author) => lines(&library.books_by(author)),
        Action::List => {
            let mut books = library.find("");
            if config.by_year {
                books.sort_by(|a, b| {
                    a.year
                        .cmp(&b.year)
                        .then_with(|| a.title.cmp(&b.title))
                        .then_with(|| a.id.cmp(&b.id))
                });
            } else {
                books.sort_by(|a, b| {
                    sort_key(&a.title)
                        .cmp(sort_key(&b.title))
                        .then_with(|| a.title.cmp(&b.title))
                        .then_with(|| a.id.cmp(&b.id))
                });
            }
            lines(&books)
        }
        Action::Authors => {
            let counts = library.find("").iter().fold(HashMap::new(), |mut map, b| {
                *map.entry(b.author.as_str()).or_insert(0usize) += 1;
                map
            });
            let mut pairs: Vec<(&str, usize)> = counts.into_iter().collect();
            pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
            pairs
                .iter()
                .map(|(author, n)| format!("{author}: {n}\n"))
                .collect()
        }
        Action::Decades => {
            let counts = library.find("").iter().fold(HashMap::new(), |mut map, b| {
                *map.entry(b.year / 100 * 100).or_insert(0usize) += 1;
                map
            });
            let mut pairs: Vec<(u16, usize)> = counts.into_iter().collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            pairs
                .iter()
                .map(|(decade, n)| format!("{decade}s: {n}\n"))
                .collect()
        }
    };
    Ok(out)
}
