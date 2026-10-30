pub fn borrowed_len(text: &String) -> usize {
    let bytes: Vec<u8> = text.bytes().collect();
    bytes.len()
}
