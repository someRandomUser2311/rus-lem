/// The outcome of one action and the credit afterwards.
fn step(old: u32, action: Purchase) -> (Outcome, u32) {
    let mut credit = old;
    let outcome = apply(&mut credit, action);
    (outcome, credit)
}
