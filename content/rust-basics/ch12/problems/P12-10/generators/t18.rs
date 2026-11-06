// 300 cases as the spec's Random tests paragraph describes (seed 0x0C00000A).
fn draw(g: &mut Lcg, alphabet: &[&str], n: u32) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push_str(alphabet[g.pick(alphabet.len() as u32) as usize]);
    }
    s
}

fn main() {
    let alphabet = ["t", "T", "o", "O", "é", "É", "東", " "];
    let mut g = Lcg(0x0C00000A);
    for _ in 0..300 {
        let ignore = g.pick(2) == 1;
        let q = g.pick(3);
        let query = draw(&mut g, &alphabet, q);
        let lines = g.pick(6);
        let mut text = String::new();
        for _ in 0..lines {
            let n = g.pick(10);
            text.push_str(&draw(&mut g, &alphabet, n));
            text.push('\n');
        }
        call(format!("crate::matching_lines({query:?}, {text:?}, {ignore})"));
    }
}
