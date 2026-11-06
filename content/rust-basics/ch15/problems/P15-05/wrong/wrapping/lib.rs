use std::ops::{Deref, DerefMut};

pub struct Meter(pub u32);

impl Deref for Meter {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for Meter {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

pub fn increase(meter: &mut Meter, by: u32) {
    **meter = meter.wrapping_add(by);
}
