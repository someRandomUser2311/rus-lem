#[derive(Debug, Clone, PartialEq)]
pub struct Stopwatch {
    pub seconds: u32,
    pub laps: u32,
}

impl Stopwatch {
    pub fn tick(&mut self, secs: u32) {
        self.seconds = self.seconds + secs;
    }

    pub fn lap(&mut self) -> u32 {
        self.laps = self.laps.saturating_add(1);
        self.laps
    }

    pub fn display(&self) -> String {
        format!(
            "{}:{:02}:{:02}",
            self.seconds / 3600,
            self.seconds / 60 % 60,
            self.seconds % 60
        )
    }

    pub fn finish(self) -> (u32, u32) {
        (self.seconds, self.laps)
    }
}
