fn check(len: usize, i: usize) -> Result<(), String> {
    if i < len {
        Ok(())
    } else {
        Err(format!("index {i} out of bounds for length {len}"))
    }
}

pub fn swap_at(values: &mut [i32], i: usize, j: usize) -> Result<(), String> {
    check(values.len(), i)?;
    check(values.len(), j)?;
    let base = values.as_mut_ptr();
    // SAFETY: i and j are in bounds; ptr::swap allows the two places to be equal.
    unsafe {
        std::ptr::swap(base.add(i), base.add(j));
    }
    Ok(())
}

pub fn rotate_three(values: &mut [i32], a: usize, b: usize, c: usize) -> Result<(), String> {
    let len = values.len();
    check(len, a)?;
    check(len, b)?;
    check(len, c)?;
    if a == b || b == c || a == c {
        return Err(String::from("indices must differ"));
    }
    let base = values.as_mut_ptr();
    // SAFETY: a, b and c are distinct and in bounds, so every read and write is valid.
    unsafe {
        let (pa, pb, pc) = (base.add(a), base.add(b), base.add(c));
        let va = std::ptr::read(pa);
        let vb = std::ptr::read(pb);
        let vc = std::ptr::read(pc);
        std::ptr::write(pb, vc);
        std::ptr::write(pc, va);
        std::ptr::write(pa, vb);
    }
    Ok(())
}
