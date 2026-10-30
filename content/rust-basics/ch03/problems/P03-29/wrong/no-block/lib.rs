pub fn statistics(values: [i32; 8]) -> (i32, i32, i64, u32, f64) {
    let mut minimum = values[0];
    let mut maximum = values[0];
    let mut positive = 0;
    let mut sum: i64 = 0;
    for value in values {
        if value < minimum {
            minimum = value;
        }
        if value > maximum {
            maximum = value;
        }
        if value > 0 {
            positive += 1;
        }
        sum += value as i64;
    }
    (minimum, maximum, sum, positive, sum as f64 / 8.0)
}
