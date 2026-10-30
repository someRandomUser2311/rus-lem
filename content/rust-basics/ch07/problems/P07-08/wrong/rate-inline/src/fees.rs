mod rate {
    pub const CENTS: u32 = 25;
}

pub fn charge(units: u32) -> u32 {
    units.saturating_mul(rate::CENTS)
}
