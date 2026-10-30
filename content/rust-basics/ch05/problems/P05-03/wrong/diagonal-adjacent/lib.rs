pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn manhattan(a: &Point, b: &Point) -> u64 {
    let x = a.x as i64 - b.x as i64;
    let y = a.y as i64 - b.y as i64;
    (x.abs() + y.abs()) as u64
}

pub fn is_adjacent(a: &Point, b: &Point) -> bool {
    (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1 && manhattan(a, b) != 0
}
