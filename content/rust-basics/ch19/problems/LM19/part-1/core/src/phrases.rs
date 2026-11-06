use crate::command::{Command, parse_command};
use crate::error::LibraryError;
use crate::fees::PREMIUM_LOAN_DAYS;

fn number(word: &str, line: &str) -> Result<u32, LibraryError> {
    word.parse::<u32>()
        .map_err(|_| LibraryError::BadCommand(line.to_string()))
}

pub fn parse_phrase(line: &str) -> Result<Command, LibraryError> {
    let bad = || LibraryError::BadCommand(line.to_string());
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        ["lend", "book", book, "to", "member", member, "on", "day", day] => {
            let today = number(day, line)?;
            if today > u32::MAX - PREMIUM_LOAN_DAYS {
                return Err(bad());
            }
            Ok(Command::Checkout {
                book_id: number(book, line)?,
                member_id: number(member, line)?,
                today,
            })
        }
        ["return", "book", book, "on", "day", day] => Ok(Command::Return {
            book_id: number(book, line)?,
            today: number(day, line)?,
        }),
        ["find", query @ ..] if !query.is_empty() => Ok(Command::Find(query.join(" "))),
        ["report"] | ["show", "report"] => Ok(Command::Report),
        _ => Err(bad()),
    }
}

pub fn parse_any(line: &str) -> Result<Command, LibraryError> {
    parse_phrase(line).or_else(|_| parse_command(line))
}

pub fn describe(command: &Command) -> String {
    match command {
        Command::Add {
            title,
            year: year @ ..=1899,
            ..
        } => format!("add the classic {title} ({year})"),
        Command::Add { title, .. } => format!("add {title}"),
        Command::Checkout {
            book_id,
            member_id,
            today: 0,
        } => format!("lend book {book_id} to member {member_id} on the first day"),
        Command::Checkout {
            book_id,
            member_id,
            today,
        } => format!("lend book {book_id} to member {member_id} on day {today}"),
        Command::Return { book_id, today } => format!("return book {book_id} on day {today}"),
        Command::Find(query) if query.is_empty() => String::from("find every book"),
        Command::Find(query) => format!("find \"{query}\""),
        Command::Report => String::from("show the report"),
    }
}
