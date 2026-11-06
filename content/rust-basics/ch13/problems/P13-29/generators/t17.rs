// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D00001D);
    let pieces = ["a", " ", "|", "#", "7", "+", "x", "東"];
    for _ in 0..300 {
        let n = g.pick(8);
        let lines: Vec<String> = (0..n).map(|_| {
            let k = g.pick(5);
            let mut s = String::new();
            for _ in 0..k { s.push_str(pieces[g.pick(8) as usize]); }
            format!("String::from({s:?})")
        }).collect();
        call(format!("{{ let v: Vec<String> = vec![{}]; crate::parse_records(&v) }}", lines.join(", ")));
    }
}
