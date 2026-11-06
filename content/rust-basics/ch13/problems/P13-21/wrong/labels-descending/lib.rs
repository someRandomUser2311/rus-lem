#[derive(Debug, PartialEq)]
pub struct Row {
    pub label: String,
    pub score: i32,
}

pub fn rank(mut rows: Vec<Row>, minimum: i32) -> Vec<Row> {
    rows.retain(|r| r.score >= minimum);
    rows.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| b.label.cmp(&a.label)));
    rows
}
