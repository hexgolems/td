use crate::algebra::{Point, Vector};
use crate::playing_state::PlayingState;

pub struct Camera {
    translate: Vector,
    offset: Vector,
}

impl Camera {

    pub fn tick(state: &mut PlayingState) {
        let time = state.time();
        state.gui.cam_mut().shake(time);
    }

    pub fn shake(&mut self, time: f32){
        self.offset += Vector::new(0.0, 0.05*(time/80.0).sin());
    }

    pub fn new() -> Self {
        return Self {
            translate: Vector::new(40.0, 40.0),
            offset: Vector::new(0.0, 0.0),
        };
    }

    pub fn world_pos(&self, p: Point) -> Point {
        return p + self.translate + Vector::new(self.offset.x.floor(), self.offset.y.floor());
    }

    pub fn ground_pos(&self, p: Point) -> Point {
        return p + ((self.translate + self.offset)/ 10.0) + Vector::new(180.0,180.0);
    }
}
