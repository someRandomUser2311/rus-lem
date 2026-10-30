fn main() {
    let mut g = Lcg(0x0300001C);
    for _ in 0..300 {
        let hi = g.next() as u64;
        let lo = g.next() as u64;
        let n = hi * 4294967296 + lo;
        let limit = g.pick(10001);
        call(format!("collatz_report({n}, {limit})"));
    }
}
