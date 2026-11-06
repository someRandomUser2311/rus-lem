// 300 cases as the spec's Random tests paragraph describes (seed 0x0C000009).
fn main() {
    let alphabet = ["a", "é", "東", " "];
    let mut g = Lcg(0x0C000009);
    for _ in 0..300 {
        let items = g.pick(4) + 1;
        let mut spec = Vec::new();
        for _ in 0..items {
            let kind = g.pick(4);
            let a = g.pick(9) + 1;
            let b = g.pick(9) + 1;
            let (a, b) = if a > b { (b, a) } else { (a, b) };
            spec.push(match kind {
                0 => format!("{a}"),
                1 => format!("{a}-{b}"),
                2 => format!("{a}-"),
                _ => format!("-{b}"),
            });
        }
        let spec = spec.join(",");
        let lines = g.pick(8);
        let mut text = String::new();
        for _ in 0..lines {
            let n = g.pick(5);
            for _ in 0..n {
                text.push_str(alphabet[g.pick(4) as usize]);
            }
            text.push('\n');
        }
        call(format!(
            "{{ let ranges = crate::parse_spec({spec:?}).expect(\"generated specs are valid\"); crate::select(&ranges, {text:?}) }}"
        ));
    }
}
