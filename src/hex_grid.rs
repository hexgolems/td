use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Hash, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Pos) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Eq for Pos {}

impl Pos {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        assert!(x + y + z == 0, "x + y + z != 0");
        return Self { x, y, z };
    }

    pub fn distance(a: Pos, b: Pos) -> usize {
        return (((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) / 2) as usize;
    }

    pub fn neighbors(&self, distance: usize) -> Vec<Pos> {
        let distance = distance as isize;
        let mut n = Vec::new();
        for x in (-distance..=distance) {
            for y in max(-distance, -x - distance)..=min(distance, -x + distance) {
                let z = -x - y;
                n.push(Pos::new(self.x + x, self.y + y, self.z + z));
            }
        }
        return n;
    }

    pub fn up(&self) -> Pos {
        if (self.z.abs() % 2 == 0) {
            return Pos::new(self.x, self.y + 1, self.z - 1);
        }
        return Pos::new(self.x + 1, self.y, self.z - 1);
    }

    pub fn down(&self) -> Pos {
        if (self.z.abs() % 2 == 0) {
            return Pos::new(self.x - 1, self.y, self.z + 1);
        }
        return Pos::new(self.x, self.y - 1, self.z + 1);
    }

    pub fn left(&self) -> Pos {
        return Pos::new(self.x - 1, self.y + 1, self.z);
    }

    pub fn right(&self) -> Pos {
        return Pos::new(self.x + 1, self.y - 1, self.z);
    }
}

pub type Id = usize;

pub trait ID {
    fn id(&self) -> Id;
}

pub struct HexGrid<T: ID> {
    pub entries: HashMap<Pos, T>,
    pub ids_to_pos: HashMap<Id, Pos>,
}

impl<T: ID> HexGrid<T> {
    pub fn new() -> Self {
        let entries = HashMap::new();
        let ids_to_pos = HashMap::new();
        return Self {
            entries,
            ids_to_pos,
        };
    }

    pub fn add(&mut self, pos: &Pos, entry: T) {
        self.ids_to_pos.insert(entry.id(), pos.clone());
        self.entries.insert(pos.clone(), entry);
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        return self.entries.get(&pos);
    }

    pub fn del(&mut self, pos: &Pos) {
        if let Some(deleted) = self.entries.remove(pos) {
            self.ids_to_pos.remove(&deleted.id());
        }
    }

    pub fn neighbors(&self, pos: &Pos, distance: usize) -> Vec<&T> {
        let mut n = Vec::new();
        for pos_i in pos.neighbors(distance) {
            if let Some(entry) = self.get(&pos_i) {
                n.push(entry);
            }
        }
        return n;
    }
}
