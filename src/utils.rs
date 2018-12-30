use ggez::graphics::Point2;

pub fn distance(p1: &Point2, p2: &Point2) -> f32 {
    (p1 - p2).norm()
}

pub fn move_to(pos: Point2, target: Point2, speed: f32) -> (Point2, bool) {
    assert!(speed > 0.0);
    let dir = target - pos;
    let len = dir.norm();
    if speed > len {
        return (target, true);
    }
    let norm = dir / len;
    let update = norm * speed;
    return (pos + update, false);
}
