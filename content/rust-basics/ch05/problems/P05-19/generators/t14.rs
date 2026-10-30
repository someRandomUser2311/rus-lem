
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
    let mut g = Lcg(0x05000013);
    for _ in 0..300 {
        let title = text(&mut g);
        let body = text(&mut g);
        let revision = g.next();
        let replacement = text(&mut g);
        call(format!("{{ let mut a = record({title:?}.to_string(), {body:?}.to_string(), {revision}); let mut v = revise(&a, {replacement:?}); a.body.push('A'); v.body.push('B'); let w = retitle(a.clone(), {replacement:?}.to_string()); (a, v, w) }}"));
    }
}
