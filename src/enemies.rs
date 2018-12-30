use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::game_state::GameState;
use crate::map::{GameMap, MapTile, WalkDir};
use crate::utils::distance;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Display {
    Zombie,
}
use self::Display::*;

pub struct Enemy {
    id: usize,
    disp: Display,
    position: graphics::Point2,
    health: usize,
    tps: f32,
}

pub enum EnemyEvent {
    Die,
    Damage(usize),
    //Slow(f32),
}

impl Enemy {
    pub fn new(position: graphics::Point2, health: usize, tps: f32) -> Self {
        return Self {
            id: 0,
            disp: Zombie,
            position,
            health,
            tps, // tiles per second
        };
    }

    pub fn tick(&mut self, map: &GameMap) {
        println!("Zombie hp: {}", self.health);
        match map.tile_at(self.position) {
            MapTile::Walk(a) => self.walk(a),
            MapTile::Spawn(a) => self.walk(a),
            _ => (),
        }
    }

    fn walk(&mut self, dir: WalkDir) {
        return match dir {
            WalkDir::Up => self.position.y -= self.tps,
            WalkDir::Down => self.position.y += self.tps,
            WalkDir::Left => self.position.x -= self.tps,
            WalkDir::Right => self.position.x += self.tps,
        };
    }
}

pub struct Enemies {
    enemies: HashMap<usize, Enemy>,
    images: HashMap<Display, graphics::Image>,
    id: usize,
}

impl Enemies {
    fn load_img(&mut self, ctx: &mut Context, disp: Display, path: &str) -> GameResult<()> {
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(disp, img);
        return Ok(());
    }

    pub fn new() -> Self {
        let enemies = HashMap::new();
        let images = HashMap::new();
        let id = 0;
        return Self {
            enemies,
            images,
            id,
        };
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.load_img(ctx, Zombie, "/enemy.png")?;
        return Ok(());
    }

    pub fn spawn(&mut self, enemy: Enemy) {
        self.enemies.insert(self.id, enemy);
        self.id += 1;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for e in self.enemies.values() {
            graphics::draw_ex(
                ctx,
                &self.images[&e.disp],
                graphics::DrawParam {
                    // src: src,
                    dest: e.position,
                    //rotation: self.zoomlevel,
                    // offset: Point2::new(-16.0, 0.0),
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
