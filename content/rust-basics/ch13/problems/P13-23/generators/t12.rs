// 300 cases as the spec's Random tests paragraph describes.
fn draw(g: &mut Lcg, n: u32) -> String {
    let chars = ["a", "b", "A", "é", " ", "\r"];
    let mut s = String::new();
    for _ in 0..n { s.push_str(chars[g.pick(6) as usize]); }
    s
}
fn main() {
    let mut g = Lcg(0x0D000017);
    for _ in 0..300 {
        let qn = g.pick(3);
        let q = draw(&mut g, qn);
        let lines = g.pick(6);
        let mut parts = Vec::new();
        for _ in 0..lines { let k = g.pick(8); parts.push(draw(&mut g, k)); }
        let mut text = parts.join("\n");
        if g.pick(2) == 1 { text.push('\n'); }
        call(format!("crate::search({q:?}, {text:?})"));
    }
}
