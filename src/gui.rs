use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, DrawMode, Point2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::env;
use std::path;

use crate::game_state::GameState;
use crate::towers::{Tower, Towers};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Images {
    Cursor,
}
use self::Images::*;

enum CursorMode {
    BuildCannon,
}

pub struct Gui {
    cursor_pos: graphics::Point2,
    images: HashMap<Images, graphics::Image>,
}

impl Gui {
    fn load_img(&mut self, ctx: &mut Context, disp: Images, path: &str) -> GameResult<()> {
        let mut img = graphics::Image::new(ctx, path)?;
        img.set_filter(graphics::FilterMode::Nearest);
        self.images.insert(disp, img);
        return Ok(());
    }

    pub fn new() -> Self {
        let images = HashMap::new();
        return Self {
            cursor_pos: graphics::Point2::new(0.0, 0.0),
            images,
        };
    }

    pub fn init(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.load_img(ctx, Cursor, "/cursor.png")?;
        return Ok(());
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            &self.images[&Cursor],
            graphics::DrawParam {
                // src: src,
                dest: self.cursor_pos,
                //rotation: self.zoomlevel,
                offset: Point2::new(1.0 / 22.0, 1.0 / 22.0),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    pub fn key_down(state: &mut GameState, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if keycode == Keycode::Up {
            state.gui.cursor_pos.y -= 4.0 * 20.0;
        }
        if keycode == Keycode::Down {
            state.gui.cursor_pos.y += 4.0 * 20.0;
        }
        if keycode == Keycode::Left {
            state.gui.cursor_pos.x -= 4.0 * 20.0;
        }
        if keycode == Keycode::Right {
            state.gui.cursor_pos.x += 4.0 * 20.0;
        }
        if keycode == Keycode::Space {
            state
                .towers
                .spawn(Tower::new(state.gui.cursor_pos, 1.0, 1.0, 0.5));
        }
    }

    pub fn tick(&mut self) {}
}
