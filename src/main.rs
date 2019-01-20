#![feature(range_contains)]
extern crate ggez;
extern crate nalgebra;
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serde_derive;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::ContextBuilder;
use std::env;
use std::path;

mod algebra;
mod assets;
mod background;
mod buffs;
mod camera;
mod card;
mod debuffs;
mod effects;
mod end_state;
mod enemies;
mod enemy;
mod event_handler;
mod gui;
mod map;
mod menu_state;
mod overlay_state;
mod player;
mod playing_state;
mod projectiles;
mod shop_overlay;
mod tower;
mod tower_stats;
mod towers;
mod utils;
mod wave;
use crate::assets::Data;
use crate::event_handler::GameState;
use crate::menu_state::MenuState;

pub fn main() {
    let c = conf::Conf::new();
    let (ctx,event_loop) = ContextBuilder::new("HexTD","coco & leex").build().expect("couldn't create game context");

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    //if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //    let mut path = path::PathBuf::from(manifest_dir);
    //    path.push("resources");
    //    ctx.filesystem.mount(&path, true);
    //}

    //println!("{}", graphics::renderer_info(ctx).unwrap());

    let mut data = Data::new();
    data.init(&mut ctx).expect("couldn't load resources");

    let mut init_state = Box::new(MenuState::new());
    init_state.set_data(data);

    let events = &mut event_handler::GameEventHandler::new(init_state);

    match event::run(&mut ctx, &mut event_loop, events) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
