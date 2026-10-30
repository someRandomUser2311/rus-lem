#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    pub minutes: u16, // minutes since midnight, always 0..1440
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let t = hours as i64 * 60 + minutes as i64;
        Clock {
            minutes: (t % 1440) as u16,
        }
    }

    pub fn add_minutes(&self, m: i32) -> Clock {
        let t = self.minutes as i64 + m as i64;
        Clock {
            minutes: ((t % 1440 + 1440) % 1440) as u16,
        }
    }

    pub fn display(&self) -> String {
        format!("{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }

    pub fn minutes_until(&self, other: &Clock) -> u16 {
        ((other.minutes as i32 - self.minutes as i32 + 1440) % 1440) as u16
    }
}
