
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
    let mut g = Lcg(0x07000005);
    for _ in 0..300 {
        let name = text(&mut g);
        let price: u32 = match hp(&mut g, 4) {
            0 => 0,
            1 => u32::MAX,
            2 => 1 + hp(&mut g, 1000),
            _ => g.next(),
        };
        call(format!(
            "{{let t=crate::model::Ticket::new({name:?},{price});(t.name().to_string(),t.fare(),t.fare().cents(),crate::summary(&t))}}"
        ));
    }
}
