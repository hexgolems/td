use crate::algebra::{Point, Position, Vector};
use crate::assets::{Data, ImgID};
use crate::dijkstra::{reconstruct_path, shortest_path, Edge};
use crate::direction::{Dir, DIRECTIONS};
use crate::playing_state::PlayingState;
use crate::tile::TileType::*;
use crate::tile::{Tile, TileType};
use crate::utils::{distance, load_specs};
use ggez::graphics::{draw, DrawParam};
use ggez::{Context, GameResult};
use rand::prelude::*;
use std::collections::HashMap;
use std::f32;
use std::ops::Range;

struct Decoration {
    pos: Point,
    disp: ImgID,
}

pub struct GameMap {
    pub xsize: usize,
    pub ysize: usize,
    pub data: Vec<Vec<Tile>>,
    decorations: Vec<Decoration>,
    images: HashMap<TileType, ImgID>,
}

impl GameMap {
    pub fn new() -> Self {
        let tiletypes = load_specs::<Vec<TileType>>("map");
        let xsize = tiletypes[0].len();
        let ysize = tiletypes.len();
        let data = tiletypes
            .iter()
            .enumerate()
            .map(|(y, outer)| {
                outer
                    .iter()
                    .enumerate()
                    .map(|(x, kind)| Tile::new_from_type(*kind, x, y))
                    .collect()
            })
            .collect();
        let decorations = vec![];
        let mut images = HashMap::new();
        for dir in DIRECTIONS.iter() {
            images.insert(Walk(*dir), ImgID::Walk(*dir));
        }
        images.insert(Build, ImgID::Hex);
        images.insert(Target, ImgID::Hex);
        images.insert(Spawn, ImgID::Hex);
        let mut res = Self {
            decorations,
            data,
            xsize,
            ysize,
            images,
        };
        res.create_decorations();
        res.path(4, 4);
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

    pub fn valid_tile_pos(&self, x: isize, y: isize) -> bool {
        if x >= 0
            && y >= 0
            && x < self.xsize as isize
            && y < self.ysize as isize
            && self.get_tile_type(x as usize, y as usize) != Empty
        {
            return true;
        }
        return false;
    }

    pub fn tile_ring(x_in: isize, y_in: isize, radius: usize) -> Vec<(isize, isize)> {
        let mut results = vec![];
        let mut x = x_in;
        let mut y = y_in;
        for _ in 0..radius {
            let (x_i, y_i) = GameMap::tile_direction_neighbor(x, y, DIRECTIONS[4]);
            x = x_i;
            y = y_i;
        }
        for i in 0..6 {
            for _ in 0..radius {
                results.push((x, y));
                let (x_i, y_i) = GameMap::tile_direction_neighbor(x, y, DIRECTIONS[i]);
                x = x_i;
                y = y_i;
            }
        }
        return results;
    }

    pub fn neighbors(&self, x: usize, y: usize, radius: usize) -> Vec<(usize, usize)> {
        let mut potential = GameMap::tile_potential_neighbors(x as isize, y as isize, radius);
        potential.retain(|(x, y)| self.valid_tile_pos(*x, *y));
        return potential
            .iter()
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect();
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

    pub fn get_tile_type(&self, x: usize, y: usize) -> TileType {
        return self.data[y][x].kind;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        return &self.data[y][x];
    }

    pub fn tile_at(&self, point: Point) -> TileType {
        let (x, y) = GameMap::tile_index_at(point);
        return self.get_tile_type(x, y);
    }

    pub fn xrange(&self) -> Range<usize> {
        return 0..self.xsize;
    }

    pub fn yrange(&self) -> Range<usize> {
        return 0..self.ysize;
    }

    pub fn is_buildable(&self, x: usize, y: usize) -> bool {
        match self.get_tile_type(x, y) {
            Build => return true,
            _ => return false,
        }
    }

    pub fn is_spawn(&self, x: usize, y: usize) -> bool {
        match self.get_tile_type(x, y) {
            Spawn => return true,
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
                let tiletype = state.map.get_tile_type(x, y);
                if tiletype != Empty {
                    draw(
                        ctx,
                        data.get_i(&state.map.images[&tiletype]),
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

    pub fn build_graph(&self) -> HashMap<Position, Vec<Edge>> {
        let mut graph = HashMap::new();
        for x in self.xrange() {
            for y in self.yrange() {
                let mut nodes = vec![];
                let current = self.get_tile(x, y);
                for neighbor in self.neighbors(x, y, 1) {
                    nodes.push(Edge {
                        position: Position::new(neighbor.0, neighbor.1),
                        cost: current.cost,
                    })
                }
                graph.insert(current.position, nodes);
            }
        }
        return graph;
    }

    pub fn target(&self) -> &Tile {
        let mut x = 0;
        let mut y = 0;
        for xi in self.xrange() {
            for yi in self.yrange() {
                if self.get_tile_type(xi, yi) == Target {
                    x = xi;
                    y = yi;
                }
            }
        }
        return self.get_tile(x, y);
    }

    pub fn path(&self, x: usize, y: usize) -> Vec<Position> {
        let graph = self.build_graph();
        let target = self.target().position;
        let start = self.get_tile(x, y).position;
        let path = reconstruct_path(shortest_path(&graph, start, target), start, target);
        return path;
    }
}
