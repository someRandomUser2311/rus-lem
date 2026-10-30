// One call per case; `prj08_case` (helpers.rs) replays the seeded draws.
fn main() {
    for i in 0..300 {
        call(format!("crate::prj08_case({i})"));
    }
}
