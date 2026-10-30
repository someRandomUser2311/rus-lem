pub mod metric {
    pub fn encode(n: i32) -> i64 {
        n as i64 * 2
    }
}

pub mod legacy {
    pub fn encode(n: i32) -> i64 {
        n as i64 * 3
    }
}

pub fn compare(n: i32) -> (i64, i64) {
    use crate::{legacy::encode as metric_encode, metric::*};
    (metric_encode(n), encode(n))
}
