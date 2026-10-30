fn main() {
    let mut g = Lcg(0x06000017);
    for _ in 0..300 {
        let len = g.pick(41);
        let mut items = Vec::new();
        for _ in 0..len {
            let op = g.pick(4);
            let n = g.pick(2001) as i32 - 1000;
            items.push(match op {
                0 => format!("Command::Set({n})"),
                1 => format!("Command::Add({n})"),
                2 => format!("Command::Mul({n})"),
                _ => "Command::Reset".to_string(),
            });
        }
        call(format!("execute(&[{}])", items.join(", ")));
    }
}
