fn main() {
    let mut g = Lcg(0x04000015);
    for _ in 0..300 {
        let n = g.pick(41);
        let v: Vec<String> = (0..n).map(|_| (g.pick(2001) as i64 - 1000).to_string()).collect();
        call(format!("slice_sum(&[{}])", v.join(", ")));
    }
}
