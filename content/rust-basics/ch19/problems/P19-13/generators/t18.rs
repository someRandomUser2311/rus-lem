// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x1300000D);
    for _ in 0..300 {
        let n = g.pick(12); let mut toks: Vec<String> = Vec::new(); for _ in 0..n { let k = g.pick(9); let t = match k { 0..=2 => { let v = g.pick(20) as i64 - 10; v.to_string() } 3 => String::from("+"), 4 => String::from("-"), 5 => String::from("*"), 6 => String::from("dup"), 7 => String::from("swap"), _ => String::from("drop") }; toks.push(t); }
        call(format!("crate::run({:?})", toks.join(" ")));
    }
}
