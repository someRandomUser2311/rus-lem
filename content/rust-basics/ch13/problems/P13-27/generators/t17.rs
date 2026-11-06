// 300 cases as the spec's Random tests paragraph describes.
fn side(g: &mut Lcg) -> Vec<String> {
    let chars = ["a", "b", "B"];
    let n = g.pick(8);
    let mut v: Vec<String> = Vec::new();
    for _ in 0..n {
        let len = g.pick(3);
        let mut s = String::new();
        for _ in 0..len { s.push_str(chars[g.pick(3) as usize]); }
        v.push(s);
    }
    v.sort();
    v
}
fn lit(v: &[String]) -> String {
    let parts: Vec<String> = v.iter().map(|s| format!("String::from({s:?})")).collect();
    format!("{{ let v: Vec<String> = vec![{}]; v }}", parts.join(", "))
}
fn main() {
    let mut g = Lcg(0x0D00001B);
    for _ in 0..300 {
        let l = side(&mut g);
        let r = side(&mut g);
        call(format!("crate::merge({}, {})", lit(&l), lit(&r)));
    }
}
