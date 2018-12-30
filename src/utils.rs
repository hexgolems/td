use ggez::graphics::Point2;

pub fn distance(p1: &Point2, p2: &Point2) -> f32 {
    (p1 - p2).norm()
}
