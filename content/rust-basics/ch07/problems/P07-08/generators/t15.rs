
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
    let mut g = Lcg(0x07000008);
    for _ in 0..300 {
        let s = text(&mut g);
        let units: u32 = match hp(&mut g, 4) {
            0 => hp(&mut g, 1000),
            1 => 171798690 + hp(&mut g, 4),
            2 => g.next(),
            _ => g.next() >> 5,
        };
        call(format!("(crate::normalized_label({s:?}),crate::charge({units}))"));
    }
}
