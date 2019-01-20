use crate::algebra::{Point, Vector};
use crate::assets::ImgID;
use crate::buffs::BuffType;
use crate::debuffs::Debuff;
use crate::map::{GameMap, MapTile, WalkDir};
use crate::utils::move_to;
use crate::wave::WaveSpec;
use rand::prelude::*;
use std::collections::HashMap;

pub struct Enemy {
    pub disp: ImgID,
    pub position: Point,
    pub health: usize,
    pub walk_speed: f32,
    pub next_walk_target: Point,
    pub reached_goal: bool,
    pub color: (f32, f32, f32),
    pub size: f32,
    pub debuffs: HashMap<BuffType, Vec<Debuff>>,
}

impl Enemy {
    pub fn new(position: Point, spec: &WaveSpec) -> Self {
        return Self {
            disp: spec.img,
            position,
            health: spec.health,
            next_walk_target: position,
            walk_speed: spec.speed,
            color: spec.color,
            size: spec.size,
            debuffs: HashMap::new(),
            reached_goal: false,
        };
    }

    pub fn tick(&mut self, map: &GameMap) {
        let (new_pos, finished) =
            move_to(self.position, self.next_walk_target, self.get_walk_speed());
        self.position = new_pos;
        if finished {
            let offset = (Vector::new(rand::thread_rng().gen(), rand::thread_rng().gen()) * 60.0)
                - Vector::new(30.0, 30.0);
            self.next_walk_target = match map.tile_at(self.position) {
                MapTile::Walk(a) => self.walk_target(a) + offset,
                MapTile::Spawn(a) => self.walk_target(a) + offset,
                MapTile::Target => {
                    self.reached_goal = true;
                    self.position
                }
                _ => self.position,
            };
        }
        self.countdown_debuffs();
        for (_, debuffs) in self.debuffs.iter_mut() {
            debuffs.retain(|debuff| debuff.cooldown > 0);
        }
    }

    pub fn countdown_debuffs(&mut self) {
        for (_, debuffs) in self.debuffs.iter_mut() {
            for debuff in debuffs.iter_mut() {
                debuff.cooldown = debuff.cooldown.saturating_sub(1);
            }
        }
    }

    pub fn get_walk_speed(&self) -> f32 {
        if let Some(freeze) = self.debuffs.get(&BuffType::Freeze) {
            if let Some(max) = freeze.iter().max_by_key(|debuff| debuff.effectiveness) {
                return self.walk_speed * ((100 - max.effectiveness) as f32 / 100.0);
            }
        }
        return self.walk_speed;
    }

    fn walk_target(&mut self, dir: WalkDir) -> Point {
        let (x, y) = GameMap::tile_index_at(self.position);
        return match dir {
            WalkDir::Up => GameMap::tile_center(x, y - 1),
            WalkDir::Down => GameMap::tile_center(x, y + 1),
            WalkDir::Left => GameMap::tile_center(x - 1, y),
            WalkDir::Right => GameMap::tile_center(x + 1, y),
        };
    }
}
