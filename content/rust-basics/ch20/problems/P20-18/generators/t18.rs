// 300 cases as the spec's Random tests paragraph describes.
fn main() {
    let mut g = Lcg(0x14000012);
    for _ in 0..300 {
        let n = g.pick(10); let mut toks: Vec<String> = Vec::new(); for _ in 0..n { if g.pick(3) < 2 { let v = g.pick(21) as i64 - 10; toks.push(v.to_string()); } else { toks.push(["+", "-", "*", "/"][g.pick(4) as usize].to_string()); } }
        call(format!("crate::eval_rpn({:?})", toks.join(" ")));
    }
}
