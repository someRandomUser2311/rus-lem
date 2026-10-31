pub struct Window<'a, T> {
    items: &'a [T],
    start: usize,
    end: usize,
}

impl<'a, T> Window<'a, T> {
    pub fn new(items: &'a [T], start: usize, end: usize) -> Option<Self> {
        if start <= end && end <= items.len() {
            Some(Self { items, start, end })
        } else {
            None
        }
    }

    pub fn view(&self) -> &[T] {
        &self.items[..self.end]
    }

    pub fn original(&self) -> &'a [T] {
        &self.items[self.start..self.end]
    }

    pub fn kind(&self) -> &'static str {
        "window"
    }
}
