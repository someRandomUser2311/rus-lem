#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
pub fn direction(text: &str) -> Option<Direction> {
    match text {
        "N" | "north" => Some(Direction::North),
        "E" | "east" => Some(Direction::East),
        "S" | "south" => Some(Direction::South),
        "W" | "west" => Some(Direction::West),
        _ => None,
    }
}
