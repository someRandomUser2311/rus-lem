pub const BYTE_CAP: u8 = 255;
pub fn capped_counter(mut start: u8, steps: u32) -> (u8, u32) {
    let mut applied = 0;
    loop {
        if applied == steps || start == BYTE_CAP {
            break (start, applied);
        }
        start = start.saturating_add(1);
        applied += 1;
    }
}
