/// Starts a machine with `stock` and `credit` and runs `|`-separated operations, logging
/// each result with the credit and stock afterwards: `i0`…`i3` insert Nickel, Dime,
/// Quarter, Dollar; `b0`…`b2` buy Tea, Coffee, Water; `r` refunds.
fn drive(stock: [u32; 3], credit: u32, ops: &str) -> String {
    #[allow(unused_imports)]
    use crate::{coin::Coin, item::Item, machine::Machine};
    let mut m = Machine { stock, credit };
    let mut log = String::new();
    for op in ops.split('|').filter(|o| !o.is_empty()) {
        let (code, arg) = op.split_at(1);
        let arg: usize = arg.parse().unwrap_or(0);
        let result = match code {
            "i" => format!("{:?}", m.insert([Coin::Nickel, Coin::Dime, Coin::Quarter, Coin::Dollar][arg])),
            "b" => format!("{:?}", m.buy([Item::Tea, Item::Coffee, Item::Water][arg])),
            _ => format!("{:?}", m.refund()),
        };
        log.push_str(&format!("{op} -> {result} {} {:?}\n", m.credit, m.stock));
    }
    log
}
