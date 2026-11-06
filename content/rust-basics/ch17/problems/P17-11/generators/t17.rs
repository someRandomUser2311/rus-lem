// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1100000B);
    for _ in 0..300 {
        let names = ["a", "b", "é"]; let n = g.pick(8); let mut js = Vec::new(); for _ in 0..n { let name = names[g.pick(3) as usize]; let size = g.pick(3); js.push(format!("crate::job({name:?}, 0, {size})")); }
        call(format!("trpl::block_on(crate::fetch_batch(vec![{}], 300))", js.join(", ")));
    }
}
