use crate::enemies::{Enemies, EnemyEvent};
use crate::game_state::GameState;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::num;
use std::path;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Display {
    Cannon,
}

use self::Display::*;

pub struct Tower {
    disp: Display,
    position: graphics::Point2,
    damage: usize,
    range: f32,
    sps: f32, // shots per second
}

impl Tower {
    pub fn new(position: graphics::Point2, damage: usize, range: f32, sps: f32) -> Self {
        return Self {
            disp: Cannon,
            position,
            damage,
            range,
            sps,
        };
    }

    pub fn tick(&mut self, enemies: &mut Enemies) {
        if let Some(id) = enemies.weakest_enemy_in_range(self.range, self.position) {
            println!("shooting: {}", id);
            enemies.send(id, EnemyEvent::Damage(self.damage));
        }
    }
}

pub struct Towers {
    towers: Vec<Tower>,
    images: HashMap<Display, graphics::Image>,
}

impl Towers {
    fn load_img(&mut self, ctx: &mut Context, disp: Display, path: &str) -> GameResult<()> {
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(disp, img);
        return Ok(());
    }

    pub fn new() -> Self {
        let towers = vec![];
        let images = HashMap::new();
        return Self { towers, images };
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.load_img(ctx, Cannon, "/cannon.png")?;
        return Ok(());
    }

    pub fn spawn(&mut self, tower: Tower) {
        self.towers.push(tower);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for e in self.towers.iter() {
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

    pub fn tick(state: &mut GameState) {
        for t in state.towers.towers.iter_mut() {
            t.tick(&mut state.enemies)
        }
    }
}
