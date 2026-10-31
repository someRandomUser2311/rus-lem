use crate::{
    error::LibraryError,
    fees::{PREMIUM_LOAN_DAYS, format_cents},
    library::Library,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Add {
        title: String,
        author: String,
        year: u16,
    },
    Checkout {
        book_id: u32,
        member_id: u32,
        today: u32,
    },
    Return {
        book_id: u32,
        today: u32,
    },
    Find(String),
    Report,
}

fn bad(line: &str) -> LibraryError {
    LibraryError::BadCommand(line.to_string())
}

fn number(word: Option<&str>, line: &str) -> Result<u32, LibraryError> {
    let word = word.ok_or(bad(line))?;
    word.parse::<u32>().ok().ok_or(bad(line))
}

fn parse_add(rest: &str, line: &str) -> Result<Command, LibraryError> {
    let mut fields = rest.split('|');
    let title = fields.next().ok_or(bad(line))?.trim();
    let author = fields.next().ok_or(bad(line))?.trim();
    let year = fields.next().ok_or(bad(line))?.trim();
    if fields.next().is_some() {
        return Err(bad(line));
    }
    let year = year.parse::<u16>().ok().ok_or(bad(line))?;
    Ok(Command::Add {
        title: title.to_string(),
        author: author.to_string(),
        year,
    })
}

pub fn parse_command(line: &str) -> Result<Command, LibraryError> {
    let text = line.trim();
    if text.contains('\n') || text.contains('\r') {
        return Err(bad(line));
    }
    let mut words = text.split_whitespace();
    let keyword = words.next().ok_or(bad(line))?;
    let rest = text[keyword.len()..].trim_start();
    match keyword {
        "add" => parse_add(rest, line),
        "checkout" => {
            let book_id = number(words.next(), line)?;
            let member_id = number(words.next(), line)?;
            let today = number(words.next(), line)?;
            if words.next().is_some() || today > u32::MAX - PREMIUM_LOAN_DAYS {
                return Err(bad(line));
            }
            Ok(Command::Checkout {
                book_id,
                member_id,
                today,
            })
        }
        "return" => {
            let book_id = number(words.next(), line)?;
            let today = number(words.next(), line)?;
            if words.next().is_some() {
                return Err(bad(line));
            }
            Ok(Command::Return { book_id, today })
        }
        "find" => {
            if rest.is_empty() {
                return Err(bad(line));
            }
            Ok(Command::Find(rest.to_string()))
        }
        "report" => {
            if words.next().is_some() {
                return Err(bad(line));
            }
            Ok(Command::Report)
        }
        _ => Err(bad(line)),
    }
}

impl Library {
    pub fn execute(&mut self, cmd: Command) -> Result<String, LibraryError> {
        match cmd {
            Command::Add {
                title,
                author,
                year,
            } => {
                let id = self.add_book(&title, &author, year);
                Ok(format!("Added book {id}."))
            }
            Command::Checkout {
                book_id,
                member_id,
                today,
            } => {
                let due = self.checkout(book_id, member_id, today)?;
                Ok(format!("Book {book_id} due on day {due}."))
            }
            Command::Return { book_id, today } => {
                let fee = self.return_book(book_id, today)?;
                Ok(format!("Fee: {}.", format_cents(fee)))
            }
            Command::Find(query) => {
                let ids = self.search_titles(&query);
                if ids.is_empty() {
                    return Ok(String::from("Found: none."));
                }
                let mut out = String::from("Found: ");
                for (i, id) in ids.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&id.to_string());
                }
                out.push('.');
                Ok(out)
            }
            Command::Report => {
                let (available, borrowed, lost) = self.count_by_status();
                Ok(format!(
                    "Available: {available}\nBorrowed: {borrowed}\nLost: {lost}\n"
                ))
            }
        }
    }
}
