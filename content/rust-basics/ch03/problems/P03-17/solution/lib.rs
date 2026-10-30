pub fn count_matches(values: [i32; 8], target: i32) -> u32 {
    let mut count = 0;
    for value in values {
        if value == target {
            count += 1;
        }
    }
    count
}
