// One call per case; `lm9_case` (helpers.rs) replays the seeded draws.
fn main() {
    for i in 0..300 {
        call(format!("lm9_case({i})"));
    }
}
