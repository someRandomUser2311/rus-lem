// 300 cases as the spec's Random tests paragraph describes (seed 0x0C000006).
fn draw(g: &mut Lcg, alphabet: &[&str], n: u32) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push_str(alphabet[g.pick(alphabet.len() as u32) as usize]);
    }
    s
}

fn main() {
    let alphabet = ["a", "A", "b", "B", "é", "É", "東", " ", "\t"];
    let mut g = Lcg(0x0C000006);
    for _ in 0..300 {
        let q = g.pick(3);
        let query = draw(&mut g, &alphabet, q);
        let lines = g.pick(6);
        let mut text = String::new();
        for _ in 0..lines {
            let n = g.pick(12);
            text.push_str(&draw(&mut g, &alphabet, n));
            text.push('\n');
        }
        call(format!("crate::search_case_insensitive({query:?}, {text:?})"));
    }
}
