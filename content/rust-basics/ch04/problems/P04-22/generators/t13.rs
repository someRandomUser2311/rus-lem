fn main() {
    let mut g = Lcg(0x04000016);
    for _ in 0..300 {
        let n = g.pick(41);
        let v: Vec<String> = (0..n).map(|_| g.next().to_string()).collect();
        let bonus = g.next();
        call(format!("{{ let mut s: [u32; {n}] = [{}]; adjust(&mut s, {bonus}); s }}", v.join(", ")));
    }
}
