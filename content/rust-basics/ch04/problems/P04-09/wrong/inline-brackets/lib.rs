pub fn wrap(gift: String) -> String {
    format!("[{gift}]")
}
pub fn wrap_all(gift: String, layers: u32) -> String {
    let mut gift = gift;
    for _ in 0..layers {
        gift = format!("[{gift}]");
    }
    gift
}
