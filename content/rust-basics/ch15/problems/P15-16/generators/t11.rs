// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000010);
    for _ in 0..300 {
        let s = g.pick(32);
        let start = g.next() >> s;
        let n = g.pick(10);
        let mut v = Vec::new();
        for _ in 0..n { let k = g.pick(32); v.push(g.next() >> k); }
        call(format!("crate::broadcast({start}, &{v:?})"));
    }
}
