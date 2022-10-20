use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rectangle {
    pub id: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Deserialize)]
pub struct Line {
    pub id: String,
    pub x: usize,
    pub y: usize,
    pub len: usize,
    pub direction: Direction,
}

#[derive(Debug, Deserialize)]
pub struct Arrow {
    pub id: String,
    pub x: usize,
    pub y: usize,
    pub len: usize,
    pub direction: Direction,
}

#[derive(Debug, Deserialize)]
pub struct Text {
    pub id: String,
    pub x: usize,
    pub y: usize,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Element {
    Rectangle(Rectangle),
    Line(Line),
    Arrow(Arrow),
    Text(Text),
}
