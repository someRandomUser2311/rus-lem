// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1000000F);
    for _ in 0..300 {
        let n = g.pick(60); let v: Vec<u8> = (0..n).map(|_| g.pick(256) as u8).collect();
        call(format!("crate::histogram(vec!{v:?})"));
    }
}
