pub fn split_two(values: &mut [i32], mid: usize) -> Option<(&mut [i32], &mut [i32])> {
    let len = values.len();
    if mid >= len {
        return None;
    }
    let ptr = values.as_mut_ptr();
    // SAFETY: mid <= len, so both ranges lie inside the slice and don't overlap.
    unsafe {
        Some((
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        ))
    }
}
