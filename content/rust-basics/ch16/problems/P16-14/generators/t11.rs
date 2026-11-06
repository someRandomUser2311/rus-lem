// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1000000E);
    for _ in 0..300 {
        let n = g.pick(16); let v: Vec<u32> = (0..n).map(|_| { let s = g.pick(32); g.next() >> s }).collect();
        call(format!("crate::shared_total(vec!{v:?})"));
    }
}
