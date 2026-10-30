pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn manhattan(a: &Point, b: &Point) -> u64 {
    let x = (a.x - b.x) as i64;
    let y = (a.y - b.y) as i64;
    (x.abs() + y.abs()) as u64
}

pub fn is_adjacent(a: &Point, b: &Point) -> bool {
    manhattan(a, b) == 1
}
