fn main() {
    let mut g = Lcg(0x06000016);
    for _ in 0..300 {
        let kind = g.pick(3);
        let n = g.next();
        let ticket = match kind {
            0 => format!("Ticket::Child {{ age: {} }}", n % 256),
            1 => format!("Ticket::Adult {{ kilometres: {n} }}"),
            _ => "Ticket::Premium".to_string(),
        };
        call(format!("fare({ticket})"));
    }
}
