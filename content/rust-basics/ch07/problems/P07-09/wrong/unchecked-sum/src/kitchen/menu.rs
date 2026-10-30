use crate::Dish;

pub fn price(dish: &Dish) -> u32 {
    match dish {
        Dish::Soup => 200,
        Dish::Salad => 350,
        Dish::Rice => 500,
    }
}
