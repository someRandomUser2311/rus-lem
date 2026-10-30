
fn hp(g: &mut Lcg, k: u32) -> u32 {
    (g.next() >> 16) % k
}
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = hp(g, 25);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[hp(g, 12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x0700000a);
    for _ in 0..300 {
        const WS: [&str; 4] = [" ", "\t", "\n", "\u{3000}"];
        let n = hp(&mut g, 11);
        let mut entries: Vec<String> = Vec::new();
        for _ in 0..n {
            if hp(&mut g, 3) == 0 {
                entries.push("None".to_string());
                continue;
            }
            let s = if hp(&mut g, 4) == 0 {
                let k = 1 + hp(&mut g, 4);
                let mut s = String::new();
                for _ in 0..k {
                    s.push_str(WS[hp(&mut g, 4) as usize]);
                }
                s
            } else {
                text(&mut g)
            };
            entries.push(format!("Some({s:?})"));
        }
        call(format!("crate::numbered_labels(&[{}])", entries.join(",")));
    }
}
