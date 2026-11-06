// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000018);
    let texts = ["a", "b", "é", "e\u{301}", ""];
    for _ in 0..300 {
        let n = g.pick(12);
        let v: Vec<String> = (0..n).map(|_| format!("{:?}", texts[g.pick(5) as usize])).collect();
        call(format!("crate::identities(&[{}])", v.join(", ")));
    }
}
