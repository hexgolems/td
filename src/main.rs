#![feature(range_contains)]
extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use std::env;
use std::path;

mod assets;
mod enemies;
mod game_state;
mod gui;
mod map;
mod towers;
mod wave;

use crate::game_state::GameState;

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
    let state = &mut GameState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
