pub mod coin;
pub mod item;
pub mod machine;

pub use coin::Coin;

pub fn price_of(item: Item) -> u32 {
    item.price()
}
pub use item::Item;
pub use machine::Machine;
