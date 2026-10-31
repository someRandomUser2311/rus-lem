#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Add(u32),
    Take(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TapeError {
    BadCommand(usize),
    Insufficient(usize),
    Overflow(usize),
}

pub fn parse_command(line: &str, number: usize) -> Result<Command, TapeError> {
    let mut tokens = line.split_whitespace();
    let name = tokens.next().ok_or(TapeError::BadCommand(number))?;
    let amount = tokens.next().ok_or(TapeError::BadCommand(number))?;
    if tokens.next().is_some() {
        return Err(TapeError::BadCommand(number));
    }
    let n = amount
        .parse::<u32>()
        .ok()
        .ok_or(TapeError::BadCommand(number))?;
    match name {
        "add" => Ok(Command::Add(n)),
        "take" => Ok(Command::Take(n)),
        _ => Err(TapeError::BadCommand(number)),
    }
}

pub fn execute_tape(start: u32, text: &str) -> Result<(u32, Vec<u32>), TapeError> {
    let mut balance = start;
    let mut history = Vec::new();
    let mut number = 0;
    for line in text.lines() {
        number += 1;
        if line.trim().is_empty() {
            continue;
        }
        let command = parse_command(line, number)?;
        balance = match command {
            Command::Add(n) => balance.saturating_add(n),
            Command::Take(n) => balance
                .checked_sub(n)
                .ok_or(TapeError::Insufficient(number))?,
        };
        history.push(balance);
    }
    Ok((balance, history))
}
