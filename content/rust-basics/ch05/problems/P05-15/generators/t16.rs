fn main() {
    let mut g = Lcg(0x0500000f);
    for _ in 0..300 {
        let h = g.next() as i32;
        let m = g.next() as i32;
        let delta = g.next() as i32;
        call(format!("{{ let c = Clock::new({h}, {m}); let d = c.add_minutes({delta}); (c, c.display(), d, d.display()) }}"));
    }
}
