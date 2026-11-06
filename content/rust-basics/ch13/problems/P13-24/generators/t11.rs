// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000018);
    let words = ["prog", "q", "", "-i", "é.txt"];
    for _ in 0..300 {
        let n = g.pick(6);
        let v: Vec<String> = (0..n).map(|_| format!("String::from({:?})", words[g.pick(5) as usize])).collect();
        call(format!("{{ let v: Vec<String> = vec![{}]; crate::Config::build(v.into_iter()) }}", v.join(", ")));
    }
}
