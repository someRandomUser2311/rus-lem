pub fn ruler(n: u32) -> String {
    let mut digits: Vec<char> = Vec::new();
    for i in 0..n {
        digits.push((b'0' + (i % 10) as u8) as char);
    }
    let mut marks = String::new();
    for d in digits {
        marks.push(d);
    }
    marks
}
