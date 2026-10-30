#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

pub fn area(r: &Rect) -> u64 {
    r.width as u64 * r.height as u64
}

pub fn largest(rects: &[Rect]) -> Rect {
    let mut best = Rect {
        width: 0,
        height: 0,
    };
    let mut first = true;
    for r in rects {
        if first || area(r) > area(&best) {
            best = *r;
            first = false;
        }
    }
    best
}

pub fn count_fitting(rects: &[Rect], space: &Rect) -> usize {
    let mut count = 0;
    for r in rects {
        if r.width <= space.width && r.height <= space.height
        {
            count += 1;
        }
    }
    count
}
