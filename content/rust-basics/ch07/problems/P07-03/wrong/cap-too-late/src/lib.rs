pub mod pricing {
    fn fee_for(units: u32) -> u32 {
        if units > 200 { 1000 } else { units * 10 }
    }

    pub fn quote(units: u32) -> u32 {
        fee_for(units)
    }
}
