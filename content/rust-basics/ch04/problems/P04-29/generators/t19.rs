fn main() {
    let mut g = Lcg(0x0400001D);
    for _ in 0..300 {
        let n = g.pick(41);
        let v: Vec<String> = (0..n).map(|_| (g.pick(7) as i64 - 3).to_string()).collect();
        call(format!("longest_run(&[{}])", v.join(", ")));
    }
}
