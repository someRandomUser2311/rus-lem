pub struct Counter {
    current: u32,
    end: u32,
    done: bool,
}

impl Counter {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            current: start,
            end,
            done: start > end,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.done {
            return None;
        }
        let n = self.current;
        self.current += 1;
        if self.current > self.end {
            self.done = true;
        }
        Some(n)
    }
}

pub fn collect_counter(start: u32, end: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for n in Counter::new(start, end) {
        out.push(n);
    }
    out
}
