// One call per case; `lm8_case` (helpers.rs) replays the seeded draws.
fn main() {
    for i in 0..300 {
        call(format!("crate::lm8_case({i})"));
    }
}
