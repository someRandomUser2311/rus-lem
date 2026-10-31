// One call per case; `lm10_case` (helpers.rs) replays the seeded draws (seed 0x0A000201).
fn main() {
    for i in 0..300 {
        call(format!("lm10_case({i})"));
    }
}
