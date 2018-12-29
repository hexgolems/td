use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::enemies::Enemies;
use crate::enemies::Enemy;
use crate::gui::Gui;
use crate::map::GameMap;
use crate::towers::Tower;
use crate::towers::Towers;

pub struct GameState {
    map: GameMap,
    enemies: Enemies,
    towers: Towers,
    gui: Gui,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut map = GameMap::new();
        map.init(ctx);
        let mut enemies = Enemies::new();
        enemies.init(ctx);
        let mut towers = Towers::new();
        towers.init(ctx);

        let mut gui = Gui::new();
        gui.init(ctx);
        let s = Self {
            map,
            enemies,
            towers,
            gui,
        };
        Ok(s)
    }

    pub fn spawn(&mut self) {
        self.enemies
            .spawn(Enemy::new(self.map.tile_pos(0, 3), 10.0));
        self.towers
            .spawn(Tower::new(self.map.tile_pos(2, 2), 1.0, 1.0, 0.5));
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            Enemies::tick(self);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.map.draw(ctx);
        self.enemies.draw(ctx);
        self.towers.draw(ctx);
        self.gui.draw(ctx);

        graphics::present(ctx);
        Ok(())
    }
}
