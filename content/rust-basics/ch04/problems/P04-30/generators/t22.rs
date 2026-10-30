
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = g.pick(41);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[g.pick(12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x0400001E);
    for _ in 0..300 {
        let t = text(&mut g);
        let marker = ['!', '?', 'é', '😀', ' '][g.pick(5) as usize];
        call(format!("transform(String::from({:?}), {:?})", t, marker));
    }
}
