fn main() {
    let mut g = Lcg(0x05000010);
    for _ in 0..300 {
        let a = g.pick(2001) as i64 - 1000;
        let b = g.pick(1000) as i64 + 1;
        let c = g.pick(2001) as i64 - 1000;
        let d = g.pick(1000) as i64 + 1;
        call(format!("{{ let x = Fraction::new({a}, {b}); let y = Fraction::new({c}, {d}); let s = x.add(&y); (x, y, s, x.mul(&y), s.display()) }}"));
    }
}
