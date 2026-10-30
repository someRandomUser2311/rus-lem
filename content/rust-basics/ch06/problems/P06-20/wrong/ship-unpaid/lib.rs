#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    New,
    Paid,
    Shipped,
    Cancelled,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Pay,
    Ship,
    Cancel,
}
pub fn transition(state: State, action: Action) -> Option<State> {
    match state {
        State::New => match action {
            Action::Pay => Some(State::Paid),
            Action::Cancel => Some(State::Cancelled),
            Action::Ship => Some(State::Shipped),
        },
        State::Paid => match action {
            Action::Ship => Some(State::Shipped),
            Action::Cancel => Some(State::Cancelled),
            _ => None,
        },
        _ => None,
    }
}
