use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2};
use ggez::{Context, GameResult};

use crate::assets::{ImgID, Imgs};
use crate::card::CardType;
use crate::game_state::GameState;
use crate::map::GameMap;
use crate::towers::{Tower, TowerType};
use crate::utils::add_mod;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CursorOp {
    Build(TowerType),
    BuffTower,
}

impl CursorOp {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CursorOp::Build(t) => t.get_image_id(),
            CursorOp::BuffTower => ImgID::CannonBall,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CursorMode {
    Map {
        x: usize,
        y: usize,
        op: CursorOp,
        slot: usize,
    },
    Hand(usize),
}

use self::CursorMode::*;

impl CursorMode {
    pub fn chancel(&self, _state: &GameState) -> Self {
        return CursorMode::Hand(0);
    }

    pub fn up(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Map { ref mut y, .. } => *y = add_mod(*y, -1, state.map.ysize),
            Hand(ref mut x) => *x = add_mod(*x, -1, state.deck.hand.len()),
        }
        return res;
    }
    pub fn down(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Map { ref mut y, .. } => *y = add_mod(*y, 1, state.map.ysize),
            Hand(ref mut x) => *x = add_mod(*x, 1, state.deck.hand.len()),
        }
        return res;
    }
    pub fn left(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Map { ref mut x, .. } => *x = add_mod(*x, -1, state.map.xsize),
            Hand(_) => {}
        }
        return res;
    }
    pub fn right(&self, state: &GameState) -> Self {
        let mut res = self.clone();
        match res {
            Map { ref mut x, .. } => *x = add_mod(*x, 1, state.map.xsize),
            Hand(_) => {}
        }
        return res;
    }
}

pub struct Gui {
    cursor_state: CursorMode,
}

impl Gui {
    pub fn new() -> Self {
        let cursor_state = CursorMode::Hand(0);
        return Self { cursor_state };
    }

    pub fn set_cursor(&mut self, c: CursorMode) {
        self.cursor_state = c;
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

    fn draw_effect_preview(
        state: &GameState,
        x: usize,
        y: usize,
        op: CursorOp,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let color = if state.map.is_buildable(x, y) && state.towers.is_buildable(x, y) {
            graphics::Color::new(0.2, 1.0, 0.2, 0.7)
        } else {
            graphics::Color::new(1.0, 0.2, 0.2, 0.7)
        };
        graphics::draw_ex(
            ctx,
            state.imgs.get(&op.get_image_id()),
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

    fn draw_cards(state: &GameState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in state.deck.hand.iter().enumerate() {
            graphics::draw_ex(
                ctx,
                state.imgs.get(&card.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(500.0, 40.0 + (i as f32) * 80.0),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    fn draw_cards_cursor(&self, slot: usize, imgs: &Imgs, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            imgs.get(&ImgID::Cursor),
            graphics::DrawParam {
                dest: Point2::new(500.0, 40.0 + (slot as f32) * 80.0),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                ..Default::default()
            },
        )?;
        Ok(())
    }
    pub fn draw(state: &GameState, ctx: &mut Context) -> GameResult<()> {
        match state.gui.cursor_state {
            CursorMode::Map { x, y, op, .. } => {
                state.gui.draw_map_cursor(x, y, &state.imgs, ctx)?;
                Gui::draw_effect_preview(state, x, y, op, ctx)?;
            }
            CursorMode::Hand(slot) => {
                state.gui.draw_cards_cursor(slot, &state.imgs, ctx)?;
            }
        }
        Gui::draw_cards(state, ctx)?;
        Ok(())
    }

    pub fn key_down(state: &mut GameState, keycode: Keycode, _keymod: Mod, _repeat: bool) {
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
        if keycode == Keycode::Escape {
            state.gui.cursor_state = state.gui.cursor_state.chancel(state);
        }
        if keycode == Keycode::Space {
            match state.gui.cursor_state {
                CursorMode::Map { x, y, slot, op } => Gui::event_activate(state, x, y, slot, op),
                CursorMode::Hand(slot) => Gui::event_select(state, slot),
            }
        }
    }

    fn event_build(state: &mut GameState, x: usize, y: usize, t: TowerType) -> bool {
        if state.map.is_buildable(x, y) && state.towers.is_buildable(x, y) {
            state.towers.spawn(Tower::new(t, (x, y)));
            state.gui.cursor_state = CursorMode::Hand(0);
            return true;
        }
        return false;
    }

    fn event_activate(state: &mut GameState, x: usize, y: usize, slot: usize, op: CursorOp) {
        let activated = match op {
            CursorOp::Build(t) => Gui::event_build(state, x, y, t),
            CursorOp::BuffTower => false,
        };
        if activated {
            state.deck.discard.push(state.deck.hand[slot]);
            state.deck.hand[slot] = CardType::Empty;
        }
    }

    fn event_select(state: &mut GameState, slot: usize) {
        let card = state.deck.hand[slot].clone();
        card.activate(state, slot);
    }
}
