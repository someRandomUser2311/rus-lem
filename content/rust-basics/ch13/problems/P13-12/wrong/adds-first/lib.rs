pub struct Steps {
    next: u32,
    end: u32,
    step: u32,
    done: bool,
}

impl Steps {
    pub fn new(start: u32, end: u32, step: u32) -> Self {
        Self {
            next: start,
            end,
            step,
            done: start > end || step == 0,
        }
    }
}

impl Iterator for Steps {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.done {
            return None;
        }
        let n = self.next;
        if n + self.step > self.end {
            self.done = true;
        } else {
            self.next += self.step;
        }
        Some(n)
    }
}

pub fn step_values(start: u32, end: u32, step: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for n in Steps::new(start, end, step) {
        out.push(n);
    }
    out
}
