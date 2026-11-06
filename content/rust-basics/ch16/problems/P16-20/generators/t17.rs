// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000014);
    for _ in 0..300 {
        let n = g.pick(20); let v: Vec<u32> = (0..n).map(|_| { let s = g.pick(32); g.next() >> s }).collect(); let w = g.pick(10);
        call(format!("crate::parallel_map(vec!{v:?}, {w}, |x| x as u64 * 3 + 1)"));
    }
}
