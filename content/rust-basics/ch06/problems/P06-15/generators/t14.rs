fn main() {
    let mut g = Lcg(0x0600000f);
    for _ in 0..300 {
        let c = ['0', '1', '5', '9', '/', '\0', 'a', 'é', '東', '😀', '９', '\u{301}'][g.pick(12) as usize];
        call(format!("decimal({c:?})"));
    }
}
