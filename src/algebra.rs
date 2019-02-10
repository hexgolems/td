pub type Point = nalgebra::Point2<f32>;
pub type Vector = nalgebra::Vector2<f32>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
}
