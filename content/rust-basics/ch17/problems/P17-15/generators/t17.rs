// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1100000F);
    for _ in 0..300 {
        let n = g.pick(20); let v: Vec<u32> = (0..n).map(|_| { let s = g.pick(32); g.next() >> s }).collect();
        call(format!("trpl::block_on(crate::mixed_jobs(vec!{v:?}))"));
    }
}
