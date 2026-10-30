mod kitchen;
mod model;
pub mod order;

pub use model::{Dish, Line};
pub use order::{receipt, total as quote};
