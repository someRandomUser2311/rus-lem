#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub total_seconds: u64,
}

impl Span {
    pub fn new(hours: u64, minutes: u64, seconds: u64) -> Span {
        Span {
            total_seconds: hours
                .saturating_mul(3600)
                .saturating_add(minutes.saturating_mul(60))
                .saturating_add(seconds),
        }
    }

    pub fn from_minutes(minutes: u64) -> Span {
        Span {
            total_seconds: minutes.saturating_mul(60),
        }
    }

    pub fn zero() -> Span {
        Span { total_seconds: 0 }
    }

    pub fn parts(&self) -> (u64, u64, u64) {
        (
            self.total_seconds / 3600,
            self.total_seconds / 60,
            self.total_seconds % 60,
        )
    }

    pub fn plus(&self, other: &Span) -> Span {
        Span {
            total_seconds: self.total_seconds.saturating_add(other.total_seconds),
        }
    }
}
