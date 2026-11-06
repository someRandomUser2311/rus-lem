// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x11010000);
    for _ in 0..300 {
        let names = ["a", "b", "é", "東"]; let n = g.pick(8); let mut ls = Vec::new(); for _ in 0..n { let name = names[g.pick(4) as usize]; let size = g.pick(4); ls.push(format!("{name} 0 {size}")); } let text = ls.join("\n");
        call(format!("crate::run({text:?}, 300)"));
    }
}
