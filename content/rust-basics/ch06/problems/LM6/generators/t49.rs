
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
    let mut g = Lcg(0x0004B003);
    for _ in 0..300 {
        let title = text(&mut g, &A16);
        let query = match g.pick(4) {
            0 => text(&mut g, &A16),
            1 => title
                .chars()
                .map(|c| match c {
                    'a' => 'A',
                    'Z' => 'z',
                    'é' => 'É',
                    c => c,
                })
                .collect(),
            2 => {
                let chars: Vec<char> = title.chars().collect();
                let n = chars.len() as u32;
                let start = g.pick(n + 1);
                let length = g.pick(n - start + 1);
                chars[start as usize..(start + length) as usize].iter().collect()
            }
            _ => String::new(),
        };
        call(format!("title_matches({title:?}, {query:?})"));
    }
}
