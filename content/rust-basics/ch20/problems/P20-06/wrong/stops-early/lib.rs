pub struct Countdown {
    n: u32,
}

pub fn countdown(n: u32) -> Countdown {
    Countdown { n }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n <= 1 {
            return None;
        }
        let v = self.n;
        self.n -= 1;
        Some(v)
    }
}
