use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};

use crate::assets::{ImgID, Imgs};
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::towers::{Tower, TowerType};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum CursorMode {
    Build {
        x: usize,
        y: usize,
        t: TowerType,
        valid: bool,
    },
}

use self::CursorMode::*;

impl CursorMode {
    fn update(&mut self, state: &GameState) {
        match self {
            Build {
                x,
                y,
                ref mut valid,
                ..
            } => *valid = state.map.is_buildable(*x, *y) && state.towers.is_buildable(*x, *y),
        }
    }

    pub fn up(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { x, ref mut y, .. } => {
                if *y > 0 && state.map.inbounds(x, *y - 1) {
                    *y -= 1;
                }
            }
        }
        res.update(state);
        return res;
    }
    pub fn down(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { x, ref mut y, .. } => {
                if state.map.inbounds(x, *y + 1) {
                    *y += 1;
                }
            }
        }
        res.update(state);
        return res;
    }
    pub fn left(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut x, y, .. } => {
                if *x > 0 && state.map.inbounds(*x - 1, y) {
                    *x -= 1;
                }
            }
        }
        res.update(state);
        return res;
    }
    pub fn right(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Build { ref mut x, y, .. } => {
                if state.map.inbounds(*x + 1, y) {
                    *x += 1;
                }
            }
        }
        res.update(state);
        return res;
    }
}

pub struct Gui {
    cursor_state: CursorMode,
}

impl Gui {
    pub fn new() -> Self {
        return Self {
            cursor_state: CursorMode::Build {
                x: 0,
                y: 0,
                t: TowerType::Archers,
                valid: false,
            },
        };
    }

    fn draw_map_cursor(
        &self,
        x: usize,
        y: usize,
        imgs: &Imgs,
        ctx: &mut Context,
    ) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            imgs.get(&ImgID::Cursor),
            graphics::DrawParam {
                // src: src,
                dest: GameMap::tile_pos(x, y),
                //rotation: self.zoomlevel,
                offset: Point2::new(1.0 / 22.0, 1.0 / 22.0),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    fn draw_build_preview(
        &self,
        x: usize,
        y: usize,
        t: TowerType,
        valid: bool,
        imgs: &Imgs,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let color = if valid {
            graphics::Color::new(0.2, 1.0, 0.2, 0.7)
        } else {
            graphics::Color::new(1.0, 0.2, 0.2, 0.7)
        };
        graphics::draw_ex(
            ctx,
            imgs.get(&t.get_image_id()),
            graphics::DrawParam {
                // src: src,
                dest: GameMap::tile_center(x, y),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                color: Some(color),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    pub fn draw(&self, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        match self.cursor_state {
            CursorMode::Build { x, y, t, valid } => {
                self.draw_map_cursor(x, y, imgs, ctx)?;
                self.draw_build_preview(x, y, t, valid, imgs, ctx)?;
            }
        }
        Ok(())
    }

    pub fn key_down(state: &mut GameState, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if keycode == Keycode::A {
            state.gui.cursor_state = CursorMode::Build {
                x: 0,
                y: 0,
                t: TowerType::Archers,
                valid: false,
            };
        }
        if keycode == Keycode::C {
            state.gui.cursor_state = CursorMode::Build {
                x: 0,
                y: 0,
                t: TowerType::Cannon,
                valid: false,
            };
        }
        if keycode == Keycode::Up {
            state.gui.cursor_state = state.gui.cursor_state.up(state);
        }
        if keycode == Keycode::Down {
            state.gui.cursor_state = state.gui.cursor_state.down(state);
        }
        if keycode == Keycode::Left {
            state.gui.cursor_state = state.gui.cursor_state.left(state);
        }
        if keycode == Keycode::Right {
            state.gui.cursor_state = state.gui.cursor_state.right(state);
        }
        if keycode == Keycode::Space {
            match state.gui.cursor_state {
                CursorMode::Build {
                    x,
                    y,
                    t,
                    valid: true,
                } => state.towers.spawn(Tower::new(t, (x, y), 100, 100.0, 0.5)),
                _ => {}
            }
        }
    }
}
