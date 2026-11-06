use crate::command::{Command, parse_command};
use crate::error::LibraryError;
use crate::fees::PREMIUM_LOAN_DAYS;

pub fn parse_phrase(line: &str) -> Result<Command, LibraryError> {
    Err(LibraryError::BadCommand(line.to_string()))
}

pub fn parse_any(line: &str) -> Result<Command, LibraryError> {
    parse_command(line)
}

pub fn describe(command: &Command) -> String {
    String::new()
}
