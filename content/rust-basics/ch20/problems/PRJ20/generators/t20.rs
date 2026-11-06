// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x14010000);
    for _ in 0..300 {
        let units = ["mm", "cm", "m", "km", "ft"]; let n = g.pick(6); let mut ls = Vec::new(); for _ in 0..n { let v = g.pick(41) as i64 - 20; let u = units[g.pick(5) as usize]; ls.push(format!("{v} {u}")); } let text = ls.join("\n");
        call(format!("crate::sum_text({text:?}).map(|t| t.to_string())"));
    }
}
