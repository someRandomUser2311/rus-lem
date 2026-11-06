// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1100000D);
    for _ in 0..300 {
        let n = g.pick(30); let v: Vec<u32> = (0..n).map(|_| g.pick(1000)).collect();
        call(format!("trpl::block_on(crate::timed_receive(vec!{v:?}, 0, 300))"));
    }
}
