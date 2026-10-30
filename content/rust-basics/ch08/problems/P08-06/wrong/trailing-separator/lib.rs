#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Number(i32),
    Text(String),
    Empty,
}

pub fn row(cells: &[Cell]) -> String {
    let mut out = String::new();
    let mut first = true;
    for cell in cells {
        let _ = first;
        first = false;
        match cell {
            Cell::Number(n) => out.push_str(&format!("N:{n}")),
            Cell::Text(t) => {
                out.push_str("T:");
                out.push_str(t);
            }
            Cell::Empty => out.push('-'),
        }
    }
    out
}
