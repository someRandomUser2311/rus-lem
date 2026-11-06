// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0D000011);
    for _ in 0..300 {
        let n = g.pick(15);
        let v: Vec<i32> = (0..n).map(|_| g.pick(21) as i32 - 10).collect();
        let t = g.pick(25) as i32 - 12;
        call(format!("crate::first_above(&{v:?}, {t})"));
    }
}
