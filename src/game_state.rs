use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::assets::{ImgID, Imgs};
use crate::enemies::Enemies;
use crate::enemies::Enemy;
use crate::gui::Gui;
use crate::map::GameMap;
use crate::towers::Tower;
use crate::towers::Towers;
use crate::wave::Wave;

pub struct GameState {
    pub imgs: Imgs,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub wave: Wave,
    pub gui: Gui,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut imgs = Imgs::new();
        imgs.init(ctx).expect("couldn't load resources");
        let map = GameMap::new();
        let enemies = Enemies::new();
        let towers = Towers::new();
        let wave = Wave::new(60, 10);
        let gui = Gui::new();

        let s = Self {
            imgs,
            map,
            enemies,
            towers,
            wave,
            gui,
        };
        Ok(s)
    }

    pub fn spawn(&mut self) {
        self.enemies
            .spawn(Enemy::new(self.map.tile_pos(0, 3), 10.0, 0.25));
        self.enemies
            .spawn(Enemy::new(self.map.tile_pos(0, 3), 10.0, 0.25));
        self.towers
            .spawn(Tower::new(self.map.tile_pos(2, 2), 1.0, 1.0, 0.5));
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            Wave::tick(self);
            Enemies::tick(self);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.map.draw(&self.imgs, ctx);
        self.enemies.draw(&self.imgs, ctx);
        self.towers.draw(&self.imgs, ctx);
        self.gui.draw(&self.imgs, ctx);

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        if keycode == Keycode::Escape {
            ctx.quit().expect("Should never fail");
        }

        Gui::key_down(self, keycode, keymod, repeat);
    }
}
