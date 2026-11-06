use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Millimeters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0.saturating_add(other.0))
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        let mm = other.0 as u64 + self.0 as u64;
        Millimeters(u32::try_from(mm).unwrap_or(u32::MAX))
    }
}

impl Mul<u32> for Meters {
    type Output = Meters;

    fn mul(self, k: u32) -> Meters {
        Meters(self.0.saturating_mul(k))
    }
}

pub fn total_mm(parts: &[Meters]) -> Millimeters {
    parts.iter().fold(Millimeters(0), |acc, &m| acc + m)
}
