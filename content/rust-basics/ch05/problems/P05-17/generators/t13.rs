
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
    let mut g = Lcg(0x05000011);
    for _ in 0..300 {
        let count = g.pick(41);
        let mut ops = String::new();
        for _ in 0..count {
            let op = match g.pick(8) {
                0 => format!("i{}", A12[g.pick(12) as usize]),
                1 => format!("t{}", text(&mut g)),
                2 => "b".to_string(),
                3 => "l".to_string(),
                4 => "r".to_string(),
                5 => "h".to_string(),
                6 => "e".to_string(),
                _ => "v".to_string(),
            };
            if !ops.is_empty() {
                ops.push('|');
            }
            ops.push_str(&op);
        }
        call(format!("drive({ops:?})"));
    }
}
