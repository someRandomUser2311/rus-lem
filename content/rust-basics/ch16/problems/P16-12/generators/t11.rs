// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1000000C);
    for _ in 0..300 {
        let b = g.pick(40); let c = g.pick(40);
        call(format!("crate::withdrawals({b}, {c})"));
    }
}
