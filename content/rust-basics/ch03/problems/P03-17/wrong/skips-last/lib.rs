pub fn count_matches(values: [i32; 8], target: i32) -> u32 {
    let mut count = 0;
    let mut i = 0;
    while i < 7 {
        if values[i] == target {
            count += 1;
        }
        i += 1;
    }
    count
}
