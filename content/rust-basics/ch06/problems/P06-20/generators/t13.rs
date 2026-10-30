fn main() {
    let mut g = Lcg(0x06000014);
    for _ in 0..300 {
        let state = ["State::New", "State::Paid", "State::Shipped", "State::Cancelled"][g.pick(4) as usize];
        let action = ["Action::Pay", "Action::Ship", "Action::Cancel"][g.pick(3) as usize];
        call(format!("transition({state}, {action})"));
    }
}
