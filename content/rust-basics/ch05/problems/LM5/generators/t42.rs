fn main() {
    let mut g = Lcg(0x05020000);
    for _ in 0..300 {
        let premium = (g.next() >> 16) % 2 == 1;
        let borrowed = (g.next() >> 16) % 256;
        let mut ops = String::new();
        for _ in 0..40 {
            ops.push(if (g.next() >> 16) % 2 == 0 { 'b' } else { 'r' });
        }
        call(format!("drive({premium}, {borrowed}, {ops:?})"));
    }
}
