#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y
        }
    }
}