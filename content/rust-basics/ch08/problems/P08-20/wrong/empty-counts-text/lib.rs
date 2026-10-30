#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Number(i32),
    Text(String),
    Empty,
}

pub fn totals(v: &[Cell]) -> (i64, String, usize) {
    let mut sum = 0i64;
    let mut text = String::new();
    let mut empty = 0;
    for cell in v {
        match cell {
            Cell::Number(n) => sum += *n as i64,
            Cell::Text(t) => {
                text.push_str(t);
                empty += 1;
            }
            Cell::Empty => empty += 1,
        }
    }
    (sum, text, empty)
}
