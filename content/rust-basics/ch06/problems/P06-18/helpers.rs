/// The result of one adjustment and the stock afterwards.
fn step(old: u32, change: Adjustment) -> (Option<u32>, u32) {
    let mut stock = old;
    let result = adjust(&mut stock, change);
    (result, stock)
}
