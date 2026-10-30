
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
    let mut g = Lcg(0x07000007);
    for _ in 0..300 {
        let label = text(&mut g);
        let code: u32 = match hp(&mut g, 4) {
            0 => 0,
            1 => u32::MAX,
            2 => hp(&mut g, 100),
            _ => g.next(),
        };
        call(format!(
            "{{let t=crate::make_ticket({label:?},{code});(t.label.clone(),t.code,crate::report::display(&t))}}"
        ));
    }
}
