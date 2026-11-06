pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
    pub label: String,
}

pub fn length2(line: &Line) -> i64 {
    let Line {
        start: Point { x: x1, y: y1 },
        end: Point { x: x2, y: y2 },
        ..
    } = line;
    let dx = (*x2 - *x1) as i64;
    let dy = *y2 as i64 - *y1 as i64;
    dx * dx + dy * dy
}

pub fn label_of(line: &Line) -> &str {
    let Line { label, .. } = line;
    label
}

pub fn on_axis(point: &Point) -> &'static str {
    match point {
        Point { x: 0, y: 0 } => "origin",
        Point { x: 0, .. } => "y axis",
        Point { y: 0, .. } => "x axis",
        _ => "neither",
    }
}
