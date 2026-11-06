// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000008);
    let names = ["a", "b", "é", ""];
    for _ in 0..300 {
        let n = g.pick(8);
        let v: Vec<String> = (0..n).map(|_| format!("{:?}", names[g.pick(4) as usize])).collect();
        call(format!("crate::swap_in(&[{}])", v.join(", ")));
    }
}
