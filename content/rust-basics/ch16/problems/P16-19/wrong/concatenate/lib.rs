use std::thread;

pub fn merge(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    let _ = (i, j);
    let mut all = a.to_vec();
    all.extend_from_slice(b);
    all
}

pub fn parallel_sorted(values: &[u32], parts: usize) -> Vec<u32> {
    if values.is_empty() {
        return Vec::new();
    }
    let count = parts.clamp(1, 8).min(values.len());
    let size = values.len().div_ceil(count);
    thread::scope(|s| {
        let handles: Vec<_> = values
            .chunks(size)
            .map(|chunk| {
                s.spawn(move || {
                    let mut run = chunk.to_vec();
                    run.sort();
                    run
                })
            })
            .collect();
        let mut result = Vec::new();
        for h in handles {
            result = merge(&result, &h.join().expect("a sorter doesn't panic"));
        }
        result
    })
}
