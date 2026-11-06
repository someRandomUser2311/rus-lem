// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x10000012);
    for _ in 0..300 {
        let s = g.pick(50); let r = g.pick(50); let w = g.pick(10);
        call(format!("crate::reserve({s}, {r}, {w})"));
    }
}
