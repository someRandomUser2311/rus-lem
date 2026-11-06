// 300 cases as the spec's Random tests paragraph describes (seed 0x0C000007).
fn main() {
    let alphabet = ["a", "é", "東", " ", "\r", "\t"];
    let ends = ["\n", "\r\n", ""];
    let mut g = Lcg(0x0C000007);
    for _ in 0..300 {
        let count = g.pick(8);
        let lines = g.pick(10);
        let mut text = String::new();
        for i in 0..lines {
            let n = g.pick(6);
            for _ in 0..n {
                text.push_str(alphabet[g.pick(6) as usize]);
            }
            let end = g.pick(3) as usize;
            // Only the last line may lack a line ending.
            text.push_str(if end == 2 && i + 1 < lines { "\n" } else { ends[end] });
        }
        call(format!("crate::head_text({text:?}, {count})"));
    }
}
