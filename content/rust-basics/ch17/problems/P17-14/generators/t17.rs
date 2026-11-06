// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1100000E);
    for _ in 0..300 {
        let p = g.pick(50); let v = g.next();
        call(format!("trpl::block_on(crate::count_yields({p}, {v}))"));
    }
}
