#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Set(i32),
    Add(i32),
    Mul(i32),
    Reset,
}
pub fn execute(commands: &[Command]) -> Option<i32> {
    let mut value = 0_i32;
    for command in commands {
        let next = match command {
            Command::Set(n) => Some(*n),
            Command::Add(n) => Some(value.wrapping_add(*n)),
            Command::Mul(n) => value.checked_mul(*n),
            Command::Reset => Some(0),
        };
        let Some(n) = next else {
            return None;
        };
        value = n;
    }
    Some(value)
}
