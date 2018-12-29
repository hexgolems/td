use ggez::graphics;
use ggez::graphics::{Point2};
use ggez::{Context, GameResult};
use std::collections::HashSet;

use crate::assets::{ImgID, Imgs};
use crate::map::GameMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum TowerType{
    Cannon,
}

impl TowerType{
    pub fn get_image_id(&self)->ImgID{
        match self{
            TowerType::Cannon => ImgID::Cannon,
        }

    }
}

pub struct Tower {
    kind: TowerType,
    map_position: (usize,usize),
    damage: f32,
    range: f32,
    sps: f32, // shots per second
}

impl Tower {
    pub fn new( map_position: (usize,usize), damage: f32, range: f32, sps: f32) -> Self {
        return Self {
            kind: TowerType::Cannon,
            map_position,
            damage,
            range,
            sps,
        };
    }
}

pub struct Towers {
    towers: Vec<Tower>,
    blocked_positions: HashSet<(usize,usize)>,
}

impl Towers {
    pub fn new() -> Self {
        let towers = vec![];
        let blocked_positions = HashSet::new();
        return Self { towers, blocked_positions};
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.blocked_positions.insert(tower.map_position.clone());
        self.towers.push(tower);
    }

    pub fn is_buildable(&self, x: usize, y: usize) -> bool {
        return !self.blocked_positions.contains(&(x,y))
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for e in self.towers.iter() {
            graphics::draw_ex(
                ctx,
                imgs.get(&e.kind.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: GameMap::tile_center(e.map_position.0, e.map_position.1),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }
}
