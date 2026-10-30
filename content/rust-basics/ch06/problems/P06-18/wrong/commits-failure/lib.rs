#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Adjustment {
    Add(u32),
    Remove(u32),
    Clear,
}
pub fn adjust(stock: &mut u32, change: Adjustment) -> Option<u32> {
    let next = match change {
        Adjustment::Add(n) => stock.checked_add(n),
        Adjustment::Remove(n) => stock.checked_sub(n),
        Adjustment::Clear => Some(0),
    };
    *stock = next.unwrap_or(0);
    next
}
