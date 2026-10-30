fn main() {
    let mut g = Lcg(0x03000011);
    for _ in 0..300 {
        let values: Vec<String> = (0..8)
            .map(|_| (g.pick(2000001) as i64 - 1000000).to_string())
            .collect();
        let target = g.pick(2000001) as i64 - 1000000;
        call(format!("count_matches([{}], {target})", values.join(", ")));
    }
}
