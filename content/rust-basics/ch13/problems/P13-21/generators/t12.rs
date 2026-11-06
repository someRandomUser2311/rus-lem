// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000015);
    let labels = ["a", "b", "B", "é", ""];
    for _ in 0..300 {
        let n = g.pick(12);
        let rows: Vec<String> = (0..n).map(|_| {
            let l = labels[g.pick(5) as usize];
            let s = g.pick(7) as i32 - 3;
            format!("crate::Row {{ label: String::from({l:?}), score: {s} }}")
        }).collect();
        let m = g.pick(9) as i32 - 4;
        call(format!("crate::rank(vec![{}], {m})", rows.join(", ")));
    }
}
