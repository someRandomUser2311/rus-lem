use crate::text::sort_key;
use crate::{Book, Library};

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

const USAGE: &str = "usage: library <catalog> <list|count|find|by> [words...]";

impl Config {
    pub fn build(args: &[String], sort: Option<String>) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from(USAGE));
        }
        let command = args[2].as_str();
        let words = &args[3..];
        let action = match command {
            "list" | "count" => {
                if !words.is_empty() {
                    return Err(format!("{command} takes no words"));
                }
                if command == "list" {
                    Action::List
                } else {
                    Action::Count
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
            other => return Err(format!("unknown command: {other}")),
        };
        Ok(Config {
            catalog_path: args[1].clone(),
            action,
            by_year: sort.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("year")),
        })
    }
}

pub fn load_catalog(text: &str) -> Result<Library, String> {
    let mut library = Library::new();
    let mut number = 0;
    for line in text.lines() {
        number += 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = Vec::new();
        for field in line.split('|') {
            fields.push(field);
        }
        let year = if fields.len() == 3 {
            fields[2].trim().parse::<u16>().ok()
        } else {
            None
        };
        let Some(year) = year else {
            return Err(format!("line {number}: expected title|author|year"));
        };
        library.add_book(fields[0], fields[1], year);
    }
    Ok(library)
}

fn before(a: &Book, b: &Book, by_year: bool) -> bool {
    if by_year {
        (a.year, &a.title, a.id) < (b.year, &b.title, b.id)
    } else {
        (sort_key(&a.title), &a.title, a.id) < (sort_key(&b.title), &b.title, b.id)
    }
}

fn line_of(book: &Book) -> String {
    format!("{}: {}\n", book.id, book.summary())
}

pub fn run(config: &Config, catalog: &str) -> Result<String, String> {
    let library = load_catalog(catalog)?;
    let mut out = String::new();
    match &config.action {
        Action::Count => {
            let (available, borrowed, lost) = library.count_by_status();
            out = format!("Available: {available}\nBorrowed: {borrowed}\nLost: {lost}\n");
        }
        Action::Find(query) => {
            for book in library.find(query) {
                out.push_str(&line_of(book));
            }
        }
        Action::By(author) => {
            for book in library.books_by(author) {
                out.push_str(&line_of(book));
            }
        }
        Action::List => {
            let mut sorted: Vec<&Book> = Vec::new();
            for book in library.find("") {
                let mut i = sorted.len();
                while i > 0 && before(book, sorted[i - 1], config.by_year) {
                    i -= 1;
                }
                sorted.insert(i, book);
            }
            for book in sorted {
                out.push_str(&line_of(book));
            }
        }
    }
    Ok(out)
}
