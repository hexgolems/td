extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

mod enemies;
mod map;
use self::enemies::Enemies;
use self::enemies::Enemy;
use self::map::GameMap;

struct MainState {
    map: GameMap,
    enemies: Enemies,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut map = GameMap::new();
        map.init(ctx);
        let mut enemies = Enemies::new();
        enemies.init(ctx);
        let s = MainState { map, enemies };
        Ok(s)
    }

    pub fn spawn(&mut self) {
        self.enemies
            .spawn(Enemy::new(self.map.tile_pos(0, 3), 10.0))
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.enemies.tick();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.map.draw(ctx);
        self.enemies.draw(ctx);

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());
    let state = &mut MainState::new(ctx).unwrap();
    state.spawn();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
