fn main() {
    let mut g = Lcg(0x06000010);
    for _ in 0..300 {
        let a = g.next();
        let b = g.next();
        let op = ["Op::Add", "Op::Sub", "Op::Mul", "Op::Div"][g.pick(4) as usize];
        call(format!("calculate({a}, {b}, {op})"));
    }
}
