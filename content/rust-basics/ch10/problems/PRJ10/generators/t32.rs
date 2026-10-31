// One call per case; `prj10_case` (helpers.rs) replays the seeded draws (seed 0x0A000101).
fn main() {
    for i in 0..300 {
        call(format!("prj10_case({i})"));
    }
}
