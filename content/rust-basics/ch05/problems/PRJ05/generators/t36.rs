fn main() {
    let mut g = Lcg(0x05010000);
    for _ in 0..300 {
        let mut ops = String::new();
        for _ in 0..40 {
            let op = g.pick(3);
            let cents = ((g.next() as u64) << 32) + g.next() as u64;
            let code = ["d", "w", "t"][op as usize];
            if !ops.is_empty() {
                ops.push('|');
            }
            ops.push_str(&format!("{code}{cents}"));
        }
        call(format!("drive({ops:?})"));
    }
}
