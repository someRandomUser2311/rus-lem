// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10010000);
    for _ in 0..300 {
        let ws = ["a", "b", "A", "é", "a.", "東"]; let seps = [" ", "\t", "  "]; let n = g.pick(10); let mut ts = Vec::new(); for _ in 0..n { let k = g.pick(8); let mut s = String::new(); for j in 0..k { if j > 0 { s.push_str(seps[g.pick(3) as usize]); } s.push_str(ws[g.pick(6) as usize]); } ts.push(format!("String::from({s:?})")); } let w = g.pick(10); let k = g.pick(4);
        call(format!("{{ let t: Vec<String> = vec![{0}]; (crate::count_all(t.clone(), {w}), crate::top(t, {w}, {k})) }}", ts.join(", ")));
    }
}
