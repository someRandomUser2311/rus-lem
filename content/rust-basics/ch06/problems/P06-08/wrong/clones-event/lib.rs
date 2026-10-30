#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
pub fn label(event: &Event) -> String {
    match event.clone() {
        Event::Quit => "quit".to_string(),
        Event::Move { x, y } => format!("move {x},{y}"),
        Event::Write(text) => format!("write {text}"),
    }
}
