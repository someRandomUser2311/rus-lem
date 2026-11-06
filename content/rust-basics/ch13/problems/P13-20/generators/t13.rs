// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000014);
    let chars = ["0", "1", "9", "+", "-", " ", "a", "\u{3000}"];
    for _ in 0..300 {
        if g.pick(10) == 0 {
            call("crate::positive_count(None)");
            continue;
        }
        let mut s = String::new();
        for _ in 0..g.pick(6) { s.push_str(chars[g.pick(8) as usize]); }
        call(format!("crate::positive_count(Some({s:?}))"));
    }
}
