use crate::assets::{Data, ImgID};
use crate::utils::load_specs;
use ggez::graphics;
use ggez::graphics::Point2;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum WalkDir {
    Up,
    Down,
    Left,
    Right,
}

use self::WalkDir::*;
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum MapTile {
    Walk(WalkDir),
    Build,
    Spawn(WalkDir),
    Target,
}
use self::MapTile::*;

pub struct GameMap {
    pub xsize: usize,
    pub ysize: usize,
    data: Vec<Vec<MapTile>>,
    images: HashMap<MapTile, ImgID>,
}

impl GameMap {
    pub fn new() -> Self {
        let data = load_specs::<Vec<MapTile>>("map");
        println!("{:?}", data);
        let xsize = data[0].len();
        let ysize = data.len();
        let mut images = HashMap::new();
        images.insert(Walk(Left), ImgID::FloorWalkLeft);
        images.insert(Walk(Right), ImgID::FloorWalkRight);
        images.insert(Walk(Up), ImgID::FloorWalkUp);
        images.insert(Walk(Down), ImgID::FloorWalkDown);
        images.insert(Build, ImgID::FloorBuild);
        images.insert(Target, ImgID::FloorTarget);
        images.insert(Spawn(Left), ImgID::FloorSpawnLeft);
        images.insert(Spawn(Right), ImgID::FloorSpawnRight);
        images.insert(Spawn(Up), ImgID::FloorSpawnUp);
        images.insert(Spawn(Down), ImgID::FloorSpawnDown);
        return Self {
            data,
            xsize,
            ysize,
            images,
        };
    }

    pub fn tile_pos(x: usize, y: usize) -> graphics::Point2 {
        return graphics::Point2::new(4.0 * 20.0 * x as f32, 4.0 * 20.0 * y as f32);
    }

    pub fn tile_center(x: usize, y: usize) -> graphics::Point2 {
        return graphics::Point2::new(4.0 * 20.0 * x as f32 + 40.0, 4.0 * 20.0 * y as f32 + 40.0);
    }

    pub fn tile_index_at(pos: graphics::Point2) -> (usize, usize) {
        return ((pos.x / 80.0) as usize, (pos.y / 80.0) as usize);
    }

    pub fn tile_at(&self, pos: graphics::Point2) -> MapTile {
        let (xi, yi) = GameMap::tile_index_at(pos);
        return self.data[yi][xi];
    }

    pub fn xrange(&self) -> Range<usize> {
        return 0..self.xsize;
    }

    pub fn yrange(&self) -> Range<usize> {
        return 0..self.ysize;
    }

    pub fn is_buildable(&self, x: usize, y: usize) -> bool {
        match self.data[y][x] {
            Build => return true,
            _ => return false,
        }
    }

    pub fn is_spawn(&self, x: usize, y: usize) -> bool {
        match self.data[y][x] {
            Spawn(_) => return true,
            _ => return false,
        }
    }

    pub fn get_spawn_points(&self) -> Vec<(usize, usize)> {
        let mut spawns = Vec::new();
        for x in self.xrange() {
            for y in self.yrange() {
                if self.is_spawn(x, y) {
                    spawns.push((x, y))
                }
            }
        }
        return spawns;
    }
    pub fn draw(&self, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for x in self.xrange() {
            for y in self.yrange() {
                graphics::draw_ex(
                    ctx,
                    data.get_i(&self.images[&self.data[y][x]]),
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
