// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000013);
    for _ in 0..300 {
        let n = g.pick(40); let v: Vec<u32> = (0..n).map(|_| g.pick(20)).collect(); let p = g.pick(10);
        call(format!("crate::parallel_sorted(&{v:?}, {p})"));
    }
}
