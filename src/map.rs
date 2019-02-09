use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::playing_state::PlayingState;
use crate::utils::{distance, load_specs};
use ggez::graphics::{draw, DrawParam};
use ggez::{Context, GameResult};
use rand::prelude::*;
use std::collections::HashMap;
use std::f32;
use std::ops::Range;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum Dir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

const DIRECTIONS: [Dir; 6] = [East, NorthEast, NorthWest, West, SouthWest, SouthEast];

use self::Dir::*;
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize)]
pub enum MapTile {
    Walk(Dir),
    Empty,
    Build,
    Spawn(Dir),
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
        images.insert(Walk(NorthEast), ImgID::Hex);
        images.insert(Walk(East), ImgID::Hex);
        images.insert(Walk(SouthEast), ImgID::Hex);
        images.insert(Walk(SouthWest), ImgID::Hex);
        images.insert(Walk(West), ImgID::Hex);
        images.insert(Walk(NorthWest), ImgID::Hex);
        images.insert(Build, ImgID::Hex);
        images.insert(Target, ImgID::Hex);
        images.insert(Spawn(NorthEast), ImgID::Hex);
        images.insert(Spawn(East), ImgID::Hex);
        images.insert(Spawn(SouthEast), ImgID::Hex);
        images.insert(Spawn(SouthWest), ImgID::Hex);
        images.insert(Spawn(West), ImgID::Hex);
        images.insert(Spawn(NorthWest), ImgID::Hex);
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

    pub fn valid_tile_pos(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x > 0 && y > 0 && x < self.xsize as isize && y < self.ysize as isize {
            return Some((x as usize, y as usize));
        }
        return None;
    }

    pub fn tile_ring(x_in: isize, y_in: isize, radius: usize) -> Vec<(isize, isize)> {
        let mut results = vec![];
        let mut x = x_in;
        let mut y = y_in;
        println!("own pos: {}, {}", x, y);
        for i in 0..radius {
            let (x_i, y_i) = GameMap::tile_direction_neighbor(x, y, DIRECTIONS[4]);
            x = x_i;
            y = y_i;
        }
        println!("walked out to pos: {}, {}", x, y);
        for i in 0..6 {
            for _ in 0..radius {
                println!("pushing: {}, {}", x, y);
                results.push((x, y));
                let (x_i, y_i) = GameMap::tile_direction_neighbor(x, y, DIRECTIONS[i]);
                x = x_i;
                y = y_i;
            }
        }
        return results;
    }

    pub fn tile_potential_neighbors(x: isize, y: isize, radius: usize) -> Vec<(isize, isize)> {
        let mut results = vec![];
        for i in 1..=radius {
            results.append(&mut GameMap::tile_ring(x, y, i));
        }
        return results;
    }

    pub fn tile_direction_neighbor(x: isize, y: isize, dir: Dir) -> (isize, isize) {
        return match (dir, y % 2 == 0) {
            (Dir::NorthEast, true) => (x, y - 1),
            (Dir::NorthEast, false) => (x + 1, y - 1),
            (Dir::SouthWest, true) => (x - 1, y + 1),
            (Dir::SouthWest, false) => (x, y + 1),
            (Dir::East, _) => (x + 1, y),
            (Dir::West, _) => (x - 1, y),
            (Dir::SouthEast, true) => (x, y + 1),
            (Dir::SouthEast, false) => (x + 1, y + 1),
            (Dir::NorthWest, true) => (x - 1, y - 1),
            (Dir::NorthWest, false) => (x, y - 1),
        };
    }

    pub fn tile_pos(x: usize, y: usize) -> Point {
        if y % 2 == 0 {
            return Point::new(69.0 * x as f32, 59.0 * y as f32);
        } else {
            return Point::new(35.0 + 69.0 * x as f32, 59.0 * y as f32);
        }
    }

    pub fn tile_center(x: usize, y: usize) -> Point {
        return GameMap::tile_pos(x, y) + Vector::new(35.5, 39.5);
    }

    pub fn tile_index_at(point: Point) -> (usize, usize) {
        let x = (point.x / 69.0) as usize;
        let y = (point.y / 59.0) as usize;
        let mut min_distance = f32::INFINITY;
        let mut rx = 0;
        let mut ry = 0;
        for xi in x.saturating_sub(1)..x + 1 {
            for yi in y.saturating_sub(1)..y + 1 {
                let distance = distance(&GameMap::tile_center(xi, yi), &point);
                if min_distance > distance {
                    min_distance = distance;
                    rx = xi;
                    ry = yi;
                }
            }
        }
        return (rx as usize, ry as usize);
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
                if state.map.get_tile_type(x, y) != Empty {
                    draw(
                        ctx,
                        data.get_i(&state.map.images[&state.map.data[y][x]]),
                        DrawParam::default().dest(state.gui.cam().pos(GameMap::tile_pos(x, y))),
                    )?;
                }
            }
        }

        for dec in state.map.decorations.iter() {
            draw(
                ctx,
                data.get_i(&dec.disp),
                DrawParam::default()
                    .dest(state.gui.cam().pos(dec.pos))
                    .scale(Vector::new(4.0, 4.0))
                    .offset(Point::new(0.5, 1.0)),
            )?;
        }
        Ok(())
    }
}
