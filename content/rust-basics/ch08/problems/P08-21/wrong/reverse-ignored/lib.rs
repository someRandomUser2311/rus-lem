#[derive(Debug, Clone, Copy)]
pub enum Command {
    Push(i32),
    Pop,
    Reverse,
    Clear,
}

pub fn apply(commands: &[Command]) -> Vec<i32> {
    let mut v = Vec::new();
    for c in commands {
        match c {
            Command::Push(n) => v.push(*n),
            Command::Pop => {
                v.pop();
            }
            Command::Reverse => {}
            Command::Clear => v.clear(),
        }
    }
    v
}
