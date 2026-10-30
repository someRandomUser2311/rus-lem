
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = g.pick(25);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[g.pick(12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x06000015);
    for _ in 0..300 {
        let present = g.pick(3) != 0;
        let t = text(&mut g);
        let suffix = text(&mut g);
        let label = if present { format!("Some({t:?}.to_string())") } else { "None".to_string() };
        call(format!("decorate({label}, {suffix:?})"));
    }
}
