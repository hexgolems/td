use crate::algebra::Position;
use crate::direction::Dir;
use std::usize;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum TileType {
    Walk(Dir),
    Build,
    Spawn,
    Target,
    Empty,
}

use self::TileType::*;

pub struct Tile {
    pub kind: TileType,
    pub cost: usize,
    pub position: Position,
}

impl Tile {
    pub fn new(kind: TileType, cost: usize, x: usize, y: usize) -> Self {
        return Self {
            kind,
            cost,
            position: Position::new(x, y),
        };
    }

    pub fn new_from_type(kind: TileType, x: usize, y: usize) -> Self {
        let cost = match kind {
            Build => 30,
            Walk(_) => 1,
            Spawn => 1,
            Target => 1,
            Empty => usize::max_value(),
        };

        return Self {
            kind,
            cost,
            position: Position::new(x, y),
        };
    }
}
