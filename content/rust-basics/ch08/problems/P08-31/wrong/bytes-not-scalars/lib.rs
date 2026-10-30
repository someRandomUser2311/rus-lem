pub fn runs(text: &str) -> Vec<(char, usize)> {
    let mut out: Vec<(char, usize)> = Vec::new();
    for b in text.bytes() {
        let c = b as char;
        let len = out.len();
        if len > 0 && out[len - 1].0 == c {
            out[len - 1].1 += 1;
        } else {
            out.push((c, 1));
        }
    }
    out
}
