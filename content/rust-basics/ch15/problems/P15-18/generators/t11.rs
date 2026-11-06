// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000012);
    let names = ["a", "b", "é", ""];
    for _ in 0..300 {
        let n = g.pick(10);
        let mut ns = Vec::new();
        let mut ks = Vec::new();
        for _ in 0..n { ns.push(format!("{:?}", names[g.pick(4) as usize])); ks.push(g.pick(2) == 1); }
        call(format!("crate::live_names(&[{}], &{ks:?})", ns.join(", ")));
    }
}
