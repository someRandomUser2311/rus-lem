use std::cell::RefCell;

pub struct Recorder {
    messages: RefCell<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder {
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn send(&self, text: &str) {
        self.messages.borrow_mut().push(text.to_string());
    }

    pub fn snapshot(&self) -> Vec<String> {
        Vec::new()
    }

    pub fn count(&self) -> usize {
        self.messages.borrow().len()
    }
}
