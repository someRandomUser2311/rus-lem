// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000010);
    for _ in 0..300 {
        let n = g.pick(20);
        let v: Vec<i32> = (0..n).map(|_| g.next() as i32).collect();
        let cap: u64 = match g.pick(3) {
            0 => g.pick(100) as u64,
            1 => g.next() as u64 * 4,
            _ => u64::MAX - g.pick(1000) as u64,
        };
        call(format!("crate::capped_sum(&{v:?}, {cap})"));
    }
}
