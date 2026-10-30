#[derive(Debug, Clone, PartialEq)]
pub struct Editor {
    pub text: String,
    pub cursor: usize, // in chars, 0..=number of chars in text
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            text: String::new(),
            cursor: 0,
        }
    }

    pub fn insert(&mut self, c: char) {
        let mut s = String::new();
        s.push(c);
        self.type_str(&s);
    }

    pub fn type_str(&mut self, s: &str) {
        let mut pos = self.text.len();
        let mut index = 0;
        for (byte, _) in self.text.char_indices() {
            if index == self.cursor {
                pos = byte;
                break;
            }
            index += 1;
        }
        let mut out = String::new();
        out.push_str(&self.text[..pos]);
        out.push_str(s);
        out.push_str(&self.text[pos..]);
        self.text = out;
        self.cursor += s.chars().count();
    }

    pub fn backspace(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let mut start = 0;
        let mut stop = self.text.len();
        let mut index = 0;
        for (byte, _) in self.text.char_indices() {
            if index == self.cursor - 1 {
                start = byte;
            }
            if index == self.cursor {
                stop = byte;
                break;
            }
            index += 1;
        }
        let mut out = String::new();
        out.push_str(&self.text[..start]);
        out.push_str(&self.text[stop..]);
        self.text = out;
        self.cursor -= 1;
        true
    }

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        self.cursor += 1;
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.text.chars().count();
    }
}
