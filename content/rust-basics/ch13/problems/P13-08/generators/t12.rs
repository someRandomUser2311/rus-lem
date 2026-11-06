// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000008);
    let chars = ["a", "é", " ", "東"];
    for _ in 0..300 {
        let mut label = String::new();
        for _ in 0..g.pick(4) { label.push_str(chars[g.pick(4) as usize]); }
        let start = g.next() as i32;
        let n = g.pick(8);
        let deltas: Vec<i32> = (0..n).map(|_| g.next() as i32).collect();
        let calls: Vec<String> = deltas.iter().map(|d| format!("c({d})")).collect();
        call(format!("{{ let mut c = crate::counter(String::from({label:?}), {start}); let out: Vec<String> = vec![{}]; out }}", calls.join(", ")));
    }
}
