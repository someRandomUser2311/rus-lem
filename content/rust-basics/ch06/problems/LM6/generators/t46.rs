// LM2's runtime contract: its literal rows 1–20, its seeded row 22 (the same generator,
// seed and count) and its large row 23. Expected output comes from the reference
// solution, whose main is LM2's reference main.
fn main() {
    stdin("4\n");
    stdin("1\n4\n");
    stdin("2\n4\n");
    stdin("3\n4\n");
    stdin("1\n2\n3\n4\n");
    stdin("");
    stdin("\n4\n");
    stdin("0\n5\n4\n");
    stdin(format!("{}\n4\n", "x".repeat(999)));
    stdin(" 2 \r\n4\n");
    stdin("4");
    stdin("4\n1\n");
    stdin("4294967296\n4\n");
    stdin("-1\n4\n");
    stdin("４\n4\n");
    stdin("　4　\n");
    stdin("+4\n");
    stdin("01\n4\n");
    stdin("2");
    stdin("   ");
    // Row 22: per case, pick(13) lines, each item by pick(12), then pick(2) for a final newline.
    let items = ["1", "2", "3", "4", "0", "5", "crab", "", " 2 ", "-1", "４", "+4"];
    let mut g = Lcg(688);
    for _ in 0..300 {
        let count = g.pick(13);
        let lines: Vec<&str> = (0..count).map(|_| items[g.pick(12) as usize]).collect();
        let mut text = lines.join("\n");
        if g.pick(2) == 1 {
            text.push('\n');
        }
        stdin(text);
    }
    // Row 23.
    stdin(format!("{}4\n", "1\n".repeat(9999)));
}
