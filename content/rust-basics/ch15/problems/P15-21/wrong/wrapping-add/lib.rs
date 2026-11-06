#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

pub fn evaluate(expr: &Expr) -> Result<i64, &'static str> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(l, r) => Ok(evaluate(l)?.wrapping_add(evaluate(r)?)),
        Expr::Sub(l, r) => evaluate(l)?.checked_sub(evaluate(r)?).ok_or("overflow"),
        Expr::Mul(l, r) => evaluate(l)?.checked_mul(evaluate(r)?).ok_or("overflow"),
        Expr::Neg(x) => evaluate(x)?.checked_neg().ok_or("overflow"),
    }
}

pub fn expression(a: i64, b: i64, c: i64) -> Expr {
    Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(a)), Box::new(Expr::Num(b)))),
        Box::new(Expr::Num(c)),
    )
}

pub fn render(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => n.to_string(),
        Expr::Add(l, r) => format!("({} + {})", render(l), render(r)),
        Expr::Sub(l, r) => format!("({} - {})", render(l), render(r)),
        Expr::Mul(l, r) => format!("({} * {})", render(l), render(r)),
        Expr::Neg(x) => format!("-{}", render(x)),
    }
}
