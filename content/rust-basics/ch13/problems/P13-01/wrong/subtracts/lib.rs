pub fn bonus(score: i32, amount: i32) -> i32 {
    let add = |n| n - amount;
    add(score)
}
