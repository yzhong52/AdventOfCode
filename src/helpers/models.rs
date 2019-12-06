#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}
