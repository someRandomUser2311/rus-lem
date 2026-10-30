
const A12: [&str; 12] = ["a", "Z", "é", "東", "😀", "e", "\u{301}", " ", "\t", "\n", "\u{3000}", "\0"];
fn text(g: &mut Lcg) -> String {
    let len = g.pick(17);
    let mut s = String::new();
    for _ in 0..len {
        s.push_str(A12[g.pick(12) as usize]);
    }
    s
}
fn main() {
    let mut g = Lcg(0x05000016);
    for _ in 0..300 {
        let title = text(&mut g);
        let body = text(&mut g);
        let replacement = text(&mut g);
        let edition = g.next();
        call(format!("{{ let mut a = Draft::new({title:?}, {body:?}); a.edition = {edition}; let mut f = a.fork({replacement:?}); a.append(\"A\"); f.append(\"B\"); let p = f.publish(); let s = p.summary(); let d = p.into_draft(); (a, s, d) }}"));
    }
}
