use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::assets::{ImgID, Imgs};
use crate::game_state::GameState;
use crate::map::{GameMap, MapTile, WalkDir};

pub struct Enemy {
    disp: ImgID,
    position: graphics::Point2,
    health: f32,
    tps: f32,
}

impl Enemy {
    pub fn new(position: graphics::Point2, health: f32, tps: f32) -> Self {
        return Self {
            disp: ImgID::Zombie,
            position,
            health,
            tps, // tiles per second
        };
    }

    pub fn tick(&mut self, map: &GameMap) {
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
    enemies: Vec<Enemy>,
}

impl Enemies {
    pub fn new() -> Self {
        let enemies = vec![];
        return Self { enemies };
    }

    pub fn spawn(&mut self, enemy: Enemy) {
        self.enemies.push(enemy);
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        for e in self.enemies.iter() {
            graphics::draw_ex(
                ctx,
                imgs.get(&e.disp),
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

    pub fn tick(state: &mut GameState) {
        for e in state.enemies.enemies.iter_mut() {
            e.tick(&state.map)
        }
    }
}
