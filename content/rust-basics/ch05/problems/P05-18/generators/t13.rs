
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = g.pick(17);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[g.pick(12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x05000012);
    for _ in 0..300 {
        let name = text(&mut g);
        let count = g.pick(41);
        let mut ops = String::new();
        for _ in 0..count {
            let op = match g.pick(4) {
                0 => format!("a{}", g.next()),
                1 => "c".to_string(),
                2 => format!("r{}", text(&mut g)),
                _ => "n".to_string(),
            };
            if !ops.is_empty() {
                ops.push('|');
            }
            ops.push_str(&op);
        }
        call(format!("drive({name:?}, {ops:?})"));
    }
}
