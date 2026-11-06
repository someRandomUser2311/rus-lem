// 300 cases as the spec's Random tests paragraph describes.
fn make(g: &mut Lcg, depth: u32) -> String {
    if depth == 0 || g.pick(3) == 0 {
        let s = g.pick(20);
        let n = ((g.next() as i32) as i64) >> s;
        return format!("crate::Expr::Num({n})");
    }
    match g.pick(4) {
        0 => { let a = make(g, depth - 1); let b = make(g, depth - 1); format!("crate::Expr::Add(Box::new({a}), Box::new({b}))") }
        1 => { let a = make(g, depth - 1); let b = make(g, depth - 1); format!("crate::Expr::Sub(Box::new({a}), Box::new({b}))") }
        2 => { let a = make(g, depth - 1); let b = make(g, depth - 1); format!("crate::Expr::Mul(Box::new({a}), Box::new({b}))") }
        _ => { let a = make(g, depth - 1); format!("crate::Expr::Neg(Box::new({a}))") }
    }
}
fn main() {
    let mut g = Lcg(0x0F000015);
    for _ in 0..300 {
        let e = make(&mut g, 6);
        call(format!("{{ let e = {e}; (crate::evaluate(&e), crate::render(&e)) }}"));
    }
}
