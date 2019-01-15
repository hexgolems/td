use ggez::graphics::{Point2, Vector2};

pub struct Camera {
    translate: Vector2,
}

impl Camera {
    pub fn new() -> Self{
        return Self{translate: Vector2::new(40.0,40.0)}; 
    }

    pub fn pos(&self, p: Point2) -> Point2 {
        return p+self.translate;
    }
}
