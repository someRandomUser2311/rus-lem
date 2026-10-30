fn main() {
    let mut g = Lcg(0x05000006);
    for _ in 0..300 {
        let r1 = g.pick(256);
        let g1 = g.pick(256);
        let b1 = g.pick(256);
        let r2 = g.pick(256);
        let g2 = g.pick(256);
        let b2 = g.pick(256);
        let code = format!("#{r1:02x}{g1:02x}{b1:02x}");
        call(format!(
            "{{ let h = to_hex(Rgb({r1},{g1},{b1})); let (ok, p) = parse_hex({code:?}); let m = mix(Rgb({r1},{g1},{b1}),Rgb({r2},{g2},{b2})); (h, ok, p.0, p.1, p.2, m.0, m.1, m.2) }}"
        ));
    }
}
