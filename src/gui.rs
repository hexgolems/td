use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};

use crate::assets::{ImgID, Imgs};
use crate::game_state::GameState;
use crate::towers::{Tower};

enum CursorMode {
    BuildCannon,
}

pub struct Gui {
    cursor_pos: graphics::Point2,
    cursor_state: CursorMode,
}

impl Gui {
    pub fn new() -> Self {
        return Self {
            cursor_pos: graphics::Point2::new(0.0, 0.0),
            cursor_state: CursorMode::BuildCannon,
        };
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            imgs.get(&ImgID::Cursor),
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
            match state.gui.cursor_state {
                CursorMode::BuildCannon => state.towers.spawn(Tower::new(state.gui.cursor_pos, 1.0, 1.0, 0.5)),
            }
        }
    }

}
