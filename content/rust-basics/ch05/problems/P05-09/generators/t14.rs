fn main() {
    let mut g = Lcg(0x05000009);
    for _ in 0..300 {
        let n = g.pick(21);
        let mut rs = String::new();
        for _ in 0..n {
            let w = g.pick(1001);
            let h = g.pick(1001);
            rs.push_str(&format!("Rect {{ width: {w}, height: {h} }},"));
        }
        let sw = g.pick(1001);
        let sh = g.pick(1001);
        call(format!(
            "{{ let rs: [Rect; {n}] = [{rs}]; (largest(&rs), count_fitting(&rs, &Rect {{ width: {sw}, height: {sh} }})) }}"
        ));
    }
}
