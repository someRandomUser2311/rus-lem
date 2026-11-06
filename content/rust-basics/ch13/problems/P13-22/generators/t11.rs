// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000016);
    for _ in 0..300 {
        let n = g.pick(12);
        let v: Vec<i32> = (0..n).map(|_| g.next() as i32).collect();
        let w = g.pick(14);
        call(format!("crate::window_sums(&{v:?}, {w})"));
    }
}
