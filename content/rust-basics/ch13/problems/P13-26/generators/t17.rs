// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D00001A);
    let regions = ["n", "s", "N", "é", ""];
    for _ in 0..300 {
        let n = g.pick(12);
        let sales: Vec<String> = (0..n).map(|_| {
            let r = regions[g.pick(5) as usize];
            let shift = g.pick(32);
            let cents = g.next() >> shift;
            let paid = g.pick(3) != 0;
            format!("crate::Sale {{ region: String::from({r:?}), cents: {cents}, paid: {paid} }}")
        }).collect();
        let min: u64 = match g.pick(3) {
            0 => 0,
            1 => g.pick(2_000_000_000) as u64,
            _ => g.next() as u64 * 2,
        };
        call(format!("{{ let s: Vec<crate::Sale> = vec![{}]; crate::revenue(&s, {min}) }}", sales.join(", ")));
    }
}
