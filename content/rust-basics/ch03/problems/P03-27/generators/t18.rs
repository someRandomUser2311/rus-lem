fn main() {
    let mut g = Lcg(0x0300001B);
    let ops = ['+', '-', '*', '/', '%', '?', '😻', '×'];
    for _ in 0..300 {
        let a = g.pick(2000001) as i64 - 1000000;
        let b = g.pick(2000001) as i64 - 1000000;
        let op = ops[g.pick(8) as usize];
        call(format!("calculate({a}, {b}, {op:?})"));
    }
}
