pub fn ruler(n: u32) -> String {
    let mut marks = String::new();
    for i in 0..n {
        let digit = (b'0' + ((i + 1) % 10) as u8) as char;
        marks.push(digit);
    }
    marks
}
