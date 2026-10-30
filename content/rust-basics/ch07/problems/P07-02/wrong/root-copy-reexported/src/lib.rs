pub fn square(n: u32) -> u64 {
    n as u64 * n as u64
}

pub mod measure {
    pub use crate::square;
}
