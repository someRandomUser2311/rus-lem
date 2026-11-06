// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x0F000017);
    for _ in 0..300 {
        let n = g.pick(12);
        let names: Vec<String> = (0..n).map(|i| format!("\"p{i}\"")).collect();
        let parents: Vec<String> = (0..n).map(|i| if g.pick(4) == 0 { String::from("None") } else { format!("Some({})", g.pick(i + 2)) }).collect();
        call(format!("crate::forest(&[{}], &[{}])", names.join(", "), parents.join(", ")));
    }
}
