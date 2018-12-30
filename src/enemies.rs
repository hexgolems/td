use crate::assets::{ImgID, Imgs};
use crate::game_state::GameState;
use crate::map::{GameMap, MapTile, WalkDir};
use crate::utils::distance;
use crate::utils::move_to;
use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use ggez::{Context, GameResult};
use rand::prelude::*;
use std::collections::HashMap;

pub struct Enemy {
    id: usize,
    disp: ImgID,
    position: graphics::Point2,
    health: usize,
    walk_speed: f32,
    next_walk_target: graphics::Point2,
}

pub enum EnemyEvent {
    Die,
    Damage(usize),
    //Slow(f32),
}

impl Enemy {
    pub fn new(position: graphics::Point2, health: usize, walk_speed: f32) -> Self {
        return Self {
            id: 0,
            disp: ImgID::Zombie,
            position,
            health,
            next_walk_target: position,
            walk_speed,
        };
    }

    pub fn tick(&mut self, map: &GameMap) {
        let (new_pos, finished) = move_to(self.position, self.next_walk_target, self.walk_speed);
        self.position = new_pos;
        if finished {
            let offset = (Vector2::new(rand::thread_rng().gen(), rand::thread_rng().gen()) * 60.0)
                - Vector2::new(30.0, 30.0);
            self.next_walk_target = match map.tile_at(self.position) {
                MapTile::Walk(a) => self.walk_target(a) + offset,
                MapTile::Spawn(a) => self.walk_target(a) + offset,
                MapTile::Target => {
                    self.reached_goal();
                    self.position
                }
                _ => self.position,
            };
        }
    }

    fn reached_goal(&mut self) {
        println!("ZOMBIE reached goal");
    }

    fn walk_target(&mut self, dir: WalkDir) -> Point2 {
        let (x, y) = GameMap::tile_index_at(self.position);
        return match dir {
            WalkDir::Up => GameMap::tile_center(x, y - 1),
            WalkDir::Down => GameMap::tile_center(x, y + 1),
            WalkDir::Left => GameMap::tile_center(x - 1, y),
            WalkDir::Right => GameMap::tile_center(x + 1, y),
        };
    }
}

pub struct Enemies {
    enemies: HashMap<usize, Enemy>,
    id: usize,
}

impl Enemies {
    pub fn new() -> Self {
        let id = 0;
        let enemies = HashMap::new();
        return Self { enemies, id };
    }

    pub fn spawn(&mut self, enemy: Enemy) {
        self.enemies.insert(self.id, enemy);
        self.id += 1;
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for e in self.enemies.values() {
            graphics::draw_ex(
                ctx,
                imgs.get(&e.disp),
                graphics::DrawParam {
                    // src: src,
                    dest: e.position, //+e.offset_in_tile,
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

    pub fn in_range(&self, pos: graphics::Point2, range: f32) -> Vec<usize> {
        self.enemies
            .iter()
            .filter(|(id, e)| distance(&pos, &e.position) <= range && e.health > 0)
            .map(|(id, e)| *id)
            .collect()
    }

    pub fn weakest_enemy_in_range(&self, range: f32, pos: graphics::Point2) -> Option<usize> {
        self.in_range(pos, range)
            .iter()
            .min_by_key(|id| self.enemies.get(id).unwrap().health)
            .cloned()
    }

    pub fn tick(state: &mut GameState) {
        for e in state.enemies.enemies.values_mut() {
            e.tick(&state.map)
        }
        state.enemies.enemies.retain(|id, e| e.health > 0);
    }

    pub fn send(&mut self, id: usize, event: EnemyEvent) {
        match event {
            EnemyEvent::Damage(a) => {
                println!("got dmg: {}", a);
                if let Some(e) = self.enemies.get_mut(&id) {
                    e.health = e.health.saturating_sub(a);
                    println!("health now: {}", e.health);
                }
            }
            EnemyEvent::Die => {
                if let Some(e) = self.enemies.get_mut(&id) {
                    e.health = 0;
                }
            }
        };
    }
}
