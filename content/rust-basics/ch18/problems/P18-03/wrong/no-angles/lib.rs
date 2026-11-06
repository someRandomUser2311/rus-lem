pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Button {
    pub label: String,
}

pub struct Label {
    pub text: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("[{}]", self.label)
    }
}

impl Draw for Label {
    fn draw(&self) -> String {
        self.text.clone()
    }
}

pub fn render(item: &dyn Draw) -> String {
    item.draw()
}
