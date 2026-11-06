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

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn run(&self) -> Vec<String> {
        self.components.iter().rev().map(|c| c.draw()).collect()
    }

    pub fn render(&self) -> String {
        self.run().join("\n")
    }
}
