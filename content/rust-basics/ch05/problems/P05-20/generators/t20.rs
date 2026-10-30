fn main() {
    let mut g = Lcg(0x05000014);
    for _ in 0..300 {
        let (w, h) = (g.next(), g.next());
        let length = g.pick(21);
        let mut spaces = String::new();
        for _ in 0..length {
            let (sw, sh) = (g.next(), g.next());
            spaces.push_str(&format!("Parcel::new({sw}, {sh}), "));
        }
        call(format!("choose(&Parcel::new({w}, {h}), &[{spaces}])"));
    }
}
