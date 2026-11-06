// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000005);
    for _ in 0..300 {
        let n = g.pick(20); let v: Vec<u32> = (0..n).map(|_| g.pick(5)).collect(); let t = g.pick(6);
        call(format!("crate::first_match(&{v:?}, {t})"));
    }
}
