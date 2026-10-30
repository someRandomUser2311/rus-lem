#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

pub fn name(sq: Square) -> String {
    format!("{}{}", (b'a' + sq.file) as char, sq.rank + 1)
}

pub fn step_up(sq: Square) -> Square {
    Square {
        rank: (sq.rank + 1).min(7),
        ..sq
    }
}

pub fn same_diagonal(a: Square, b: Square) -> bool {
    a.file == b.file || a.rank == b.rank
}
