// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000019);
    let words = ["a", "b", "A", "é", "", "a b"];
    for _ in 0..300 {
        let n = g.pick(16);
        let v: Vec<String> = (0..n).map(|_| format!("String::from({:?})", words[g.pick(6) as usize])).collect();
        call(format!("{{ let v: Vec<String> = vec![{}]; crate::counts(&v) }}", v.join(", ")));
    }
}
