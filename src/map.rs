use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::playing_state::PlayingState;
use crate::utils::load_specs;
use ggez::graphics::{draw, DrawParam};
use ggez::{Context, GameResult};
use rand::prelude::*;
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

struct Decoration {
    pos: Point,
    disp: ImgID,
}

pub struct GameMap {
    pub xsize: usize,
    pub ysize: usize,
    data: Vec<Vec<MapTile>>,
    decorations: Vec<Decoration>,
    images: HashMap<MapTile, ImgID>,
}

impl GameMap {
    pub fn new() -> Self {
        let data = load_specs::<Vec<MapTile>>("map");
        let xsize = data[0].len();
        let ysize = data.len();
        let decorations = vec![];
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
        let mut res = Self {
            decorations,
            data,
            xsize,
            ysize,
            images,
        };
        res.create_decorations();
        return res;
    }

    pub fn create_decorations(&mut self) {
        let decoration_build = vec![ImgID::Tree1, ImgID::Tree2, ImgID::Tree3];
        let decoration_walk = vec![
            ImgID::Stone(1),
            ImgID::Stone(2),
            ImgID::Stone(2),
            ImgID::Stone(3),
            ImgID::Stone(4),
            ImgID::Stone(4),
        ];
        for x in self.xrange() {
            for y in self.yrange() {
                match self.get_tile_type(x, y) {
                    Build => {
                        if rand::thread_rng().gen::<f32>() > 0.1 {
                            let offset =
                                (Vector::new(rand::thread_rng().gen(), rand::thread_rng().gen())
                                    * 60.0)
                                    - Vector::new(30.0, 30.0);
                            let pos = GameMap::tile_center(x, y) + offset;
                            self.decorations.push(Decoration {
                                pos,
                                disp: decoration_build
                                    [rand::thread_rng().gen::<usize>() % decoration_build.len()],
                            });
                        }
                    }
                    Walk(_) => {
                        for _i in 1..4 {
                            if rand::thread_rng().gen::<f32>() > 0.1 {
                                let offset = (Vector::new(
                                    rand::thread_rng().gen(),
                                    rand::thread_rng().gen(),
                                ) * 70.0)
                                    - Vector::new(35.0, 30.0);
                                let pos = GameMap::tile_center(x, y) + offset;
                                self.decorations.push(Decoration {
                                    pos,
                                    disp: decoration_walk
                                        [rand::thread_rng().gen::<usize>() % decoration_walk.len()],
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn tile_pos(x: usize, y: usize) -> Point {
        return Point::new(4.0 * 20.0 * x as f32, 4.0 * 20.0 * y as f32);
    }

    pub fn tile_center(x: usize, y: usize) -> Point {
        return Point::new(4.0 * 20.0 * x as f32 + 40.0, 4.0 * 20.0 * y as f32 + 40.0);
    }

    pub fn tile_index_at(pos: Point) -> (usize, usize) {
        return ((pos.x / 80.0) as usize, (pos.y / 80.0) as usize);
    }

    pub fn get_tile_type(&self, x: usize, y: usize) -> MapTile {
        return self.data[y][x];
    }
    pub fn tile_at(&self, pos: Point) -> MapTile {
        let (xi, yi) = GameMap::tile_index_at(pos);
        return self.get_tile_type(xi, yi);
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
    pub fn draw(state: &PlayingState, data: &Data, ctx: &mut Context) -> GameResult<()> {
        for x in state.map.xrange() {
            for y in state.map.yrange() {
                draw(
                    ctx,
                    data.get_i(&state.map.images[&state.map.data[y][x]]),
                    DrawParam {
                        // src: src,
                        dest: state.gui.cam().pos(GameMap::tile_pos(x, y)),
                        //rotation: self.zoomlevel,
                        // offset: Point::new(-16.0, 0.0),
                        scale: Vector::new(4.0, 4.0),
                        // shear: shear,
                        ..Default::default()
                    },
                )?;
            }
        }

        for dec in state.map.decorations.iter() {
            draw(
                ctx,
                data.get_i(&dec.disp),
                DrawParam {
                    dest: state.gui.cam().pos(dec.pos),
                    scale: Vector::new(4.0, 4.0),
                    offset: Point::new(0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }
}
