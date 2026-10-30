fn main() {
    let dish = "Pancakes";
    let price = 450;
    let stars = 4;

    println!("=========================");
    println!("  Today's special");
    println!("=========================");
    println!("Dish: {dish}");
    println!("Price: {price} cents");
    print!("Rating: ");
    println!("{stars} stars");
    println!("Order code: {{{dish}-{price}}}");
    println!("To change the dish, edit {dish} in the code.");
    println!("=========================");
}
