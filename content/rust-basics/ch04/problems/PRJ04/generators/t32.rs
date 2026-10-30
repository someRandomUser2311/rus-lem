
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
const A16: [&str; 16] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0", "İ", "Σ", "ß", "ς"];
fn text(g: &mut Lcg, alphabet: &[&str]) -> String {
    let len = g.pick(41);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(alphabet[g.pick(alphabet.len() as u32) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x0004A002);
    for _ in 0..300 {
        call(format!("first_word({:?})", text(&mut g, &A12)));
    }
}
