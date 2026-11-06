// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x11000004);
    for _ in 0..300 {
        let a = ["a", "é", "東", "😀", " ", "\u{301}"]; let n = g.pick(6); let mut ts = Vec::new(); for _ in 0..n { let k = g.pick(5); let mut s = String::new(); for _ in 0..k { s.push_str(a[g.pick(6) as usize]); } ts.push(format!("String::from({s:?})")); }
        call(format!("crate::lengths(vec![{}])", ts.join(", ")));
    }
}
