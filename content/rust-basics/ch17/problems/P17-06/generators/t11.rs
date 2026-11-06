// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x11000006);
    for _ in 0..300 {
        let s = g.pick(32); let a = g.next() >> s; let s = g.pick(32); let b = g.next() >> s;
        call(format!("trpl::block_on(crate::both({a}, {b}))"));
    }
}
