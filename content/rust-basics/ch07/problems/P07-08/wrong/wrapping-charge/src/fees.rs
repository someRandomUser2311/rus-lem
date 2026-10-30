mod rate;

pub fn charge(units: u32) -> u32 {
    units.wrapping_mul(rate::CENTS)
}
