// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D00001E);
    let pieces = ["ab", "c", " ", "  ", "\t", "東", "\u{3000}"];
    for _ in 0..300 {
        let n = g.pick(6);
        let lines: Vec<String> = (0..n).map(|_| {
            let k = g.pick(8);
            let mut s = String::new();
            for _ in 0..k { s.push_str(pieces[g.pick(7) as usize]); }
            format!("String::from({s:?})")
        }).collect();
        call(format!("{{ let v: Vec<String> = vec![{}]; crate::collect_words(&v).iter().map(|w| w.to_string()).collect::<Vec<String>>() }}", lines.join(", ")));
    }
}
