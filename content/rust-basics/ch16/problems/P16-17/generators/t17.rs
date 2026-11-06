// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000011);
    for _ in 0..300 {
        let a = ["a", "b", "é", "東", " "]; let n = g.pick(12); let mut ts = Vec::new(); for _ in 0..n { let k = g.pick(6); let mut s = String::new(); for _ in 0..k { s.push_str(a[g.pick(5) as usize]); } ts.push(format!("String::from({s:?})")); } let w = g.pick(10);
        call(format!("crate::reverse_all(vec![{}], {w})", ts.join(", ")));
    }
}
