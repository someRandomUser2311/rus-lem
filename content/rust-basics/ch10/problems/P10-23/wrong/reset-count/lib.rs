#[derive(Debug, PartialEq)]
pub struct Slot<T> {
    pub value: T,
    pub revisions: u32,
}

impl<T> Slot<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            revisions: 0,
        }
    }

    pub fn replace(self, value: T) -> (Self, T) {
        let next = Self {
            value,
            revisions: 1,
        };
        (next, self.value)
    }
}
