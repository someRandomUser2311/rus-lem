pub fn swap_pair(number: i32, marker: char) -> (char, i32) {
    (marker, number.wrapping_abs())
}
