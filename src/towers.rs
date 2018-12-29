use ggez::graphics;
use ggez::graphics::{Point2};
use ggez::{Context, GameResult};

use crate::assets::{ImgID, Imgs};

pub struct Tower {
    disp: ImgID,
    position: graphics::Point2,
    damage: f32,
    range: f32,
    sps: f32, // shots per second
}

impl Tower {
    pub fn new(position: graphics::Point2, damage: f32, range: f32, sps: f32) -> Self {
        return Self {
            disp: ImgID::Cannon,
            position,
            damage,
            range,
            sps,
        };
    }
}

pub struct Towers {
    towers: Vec<Tower>,
}

impl Towers {
    pub fn new() -> Self {
        let towers = vec![];
        return Self { towers };
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.towers.push(tower);
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for e in self.towers.iter() {
            graphics::draw_ex(
                ctx,
                imgs.get(&e.disp),
                graphics::DrawParam {
                    // src: src,
                    dest: e.position,
                    //rotation: self.zoomlevel,
                    // offset: Point2::new(-16.0, 0.0),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }
}
