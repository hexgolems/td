use crate::algebra::{Point, Vector};

pub struct Camera {
    translate: Vector,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            translate: Vector::new(40.0, 40.0),
        };
    }

    pub fn pos(&self, p: Point) -> Point {
        return p + self.translate;
    }
}
