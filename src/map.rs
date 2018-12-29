use crate::assets::{ImgID, Imgs};
use ggez::graphics;
use ggez::graphics::{Point2};
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum WalkDir {
    Up,
    Down,
    Left,
    Right,
}

use self::WalkDir::*;
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum MapTile {
    Walk(WalkDir),
    Build,
    Spawn(WalkDir),
    Target,
}
use self::MapTile::*;

pub struct GameMap {
    xrange: Range<usize>,
    yrange: Range<usize>,
    data: Vec<Vec<MapTile>>,
    images: HashMap<MapTile, ImgID>,
}

impl GameMap {
    pub fn new() -> Self {
        let data = vec![
            vec![Target, Build, Build, Build],
            vec![Walk(Up), Walk(Left), Walk(Left), Walk(Left)],
            vec![Build, Build, Build, Walk(Up)],
            vec![Spawn(Right), Walk(Right), Walk(Right), Walk(Up)],
        ];
        let xrange = 0..4;
        let yrange = 0..4;
        let mut images = HashMap::new();
        images.insert(Walk(Left),   ImgID::FloorWalkLeft);
        images.insert(Walk(Right),  ImgID::FloorWalkRight);
        images.insert(Walk(Up),     ImgID::FloorWalkUp);
        images.insert(Walk(Down),   ImgID::FloorWalkDown);
        images.insert(Build,        ImgID::FloorBuild);
        images.insert(Target,       ImgID::FloorTarget);
        images.insert(Spawn(Left),  ImgID::FloorSpawnLeft);
        images.insert(Spawn(Right), ImgID::FloorSpawnRight);
        images.insert(Spawn(Up),    ImgID::FloorSpawnUp);
        images.insert(Spawn(Down),  ImgID::FloorSpawnDown);
        return Self { data, xrange, yrange, images };
    }

    pub fn tile_pos(x: usize, y: usize) -> graphics::Point2 {
        return graphics::Point2::new(4.0 * 20.0 * x as f32, 4.0 * 20.0 * y as f32);
    }

    pub fn tile_center(x: usize, y: usize) -> graphics::Point2 {
        return graphics::Point2::new(4.0 * 20.0 * x as f32 + 40.0, 4.0 * 20.0 * y as f32+ 40.0);
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        return self.xrange.contains(&x) && self.yrange.contains(&y)
    }

    pub fn tile_at(&self, pos: graphics::Point2) -> MapTile {
        return self.data[(pos.y / 80.0) as usize][(pos.x / 80.0) as usize];
    }

    pub fn xrange(&self) -> Range<usize>{
        return self.xrange.clone();
    }

    pub fn yrange(&self) -> Range<usize>{
        return self.yrange.clone();
    }

    pub fn is_buildable(&self, x: usize, y:usize) -> bool {
        match self.data[y][x] {
            Build => return true,
            _ => return false
        }
    }

    pub fn is_spawn(&self, x: usize, y: usize) -> bool{
        match self.data[y][x] {
            Spawn(_) => return true,
            _ => return false
        }
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for x in self.xrange() {
            for y in self.yrange() {
                graphics::draw_ex(
                    ctx,
                    imgs.get(&self.images[&self.data[y][x]]),
                    graphics::DrawParam {
                        // src: src,
                        dest: GameMap::tile_pos(x, y),
                        //rotation: self.zoomlevel,
                        // offset: Point2::new(-16.0, 0.0),
                        scale: Point2::new(4.0, 4.0),
                        // shear: shear,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }
}
