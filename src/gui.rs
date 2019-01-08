use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2, Text};
use ggez::{Context, GameResult};

use crate::assets::{Data, ImgID};
use crate::card::CardType;
use crate::map::GameMap;
use crate::playing_state::PlayingState;
use crate::utils::add_mod;
use crate::wave::WaveStatus;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CursorMode {
    Map {
        x: usize,
        y: usize,
        card: CardType,
        slot: usize,
    },
    Hand(usize),
}

use self::CursorMode::*;

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

    pub fn chancel(state: &mut PlayingState) {
        state.gui.set_cursor(CursorMode::Hand(0));
    }

    pub fn move_cursor(state: &mut PlayingState, ix: isize, iy: isize) {
        let len = state.player().deck.hand.len().clone();
        match state.gui.cursor_state {
            Map {
                ref mut x,
                ref mut y,
                ..
            } => {
                *y = add_mod(*y, iy, state.map.ysize);
                *x = add_mod(*x, ix, state.map.xsize);
            }
            Hand(ref mut slot) => {
                *slot = add_mod(*slot, iy, len);
            }
        }
    }

    pub fn set_cursor_card_effect(&mut self, slot: usize, c: &CardType) {
        self.set_cursor(CursorMode::Map {
            x: 0,
            y: 0,
            slot,
            card: c.clone(),
        });
    }

    fn draw_map_cursor(
        &self,
        x: usize,
        y: usize,
        data: &Data,
        ctx: &mut Context,
    ) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            data.get_i(&ImgID::Cursor),
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
        state: &PlayingState,
        x: usize,
        y: usize,
        card: CardType,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let color = if card.is_applicable(state, x, y) {
            graphics::Color::new(0.2, 1.0, 0.2, 0.7)
        } else {
            graphics::Color::new(1.0, 0.2, 0.2, 0.7)
        };
        graphics::draw_ex(
            ctx,
            state
                .data
                .as_ref()
                .unwrap()
                .get_i(&card.get_preview_image_id()),
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

    fn draw_cards(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in state.player().deck.hand.iter().enumerate() {
            graphics::draw_ex(
                ctx,
                state.data.as_ref().unwrap().get_i(&ImgID::Card),
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(750.0, 40.0 + (i as f32) * 80.0),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;

            graphics::draw_ex(
                ctx,
                state.data.as_ref().unwrap().get_i(&card.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(750.0, 40.0 + (i as f32) * 80.0),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(4.0, 4.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;

            let cost = card.activation_cost(state);
            if cost > 0 {
                let font = state.data.as_ref().unwrap().get_font();

                let mut desc = Text::new(ctx, &format!("{}", cost), font)?;
                desc.set_filter(graphics::FilterMode::Nearest);

                graphics::draw_ex(
                    ctx,
                    &desc,
                    graphics::DrawParam {
                        // src: src,
                        dest: Point2::new(780.0, 80.0 + (i as f32) * 80.0),
                        //rotation: self.zoomlevel,
                        offset: Point2::new(1.0, 1.0),
                        scale: Point2::new(1.0, 1.0),
                        // shear: shear,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    fn draw_cards_cursor(&self, slot: usize, data: &Data, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            data.get_i(&ImgID::Cursor),
            graphics::DrawParam {
                dest: Point2::new(750.0, 40.0 + (slot as f32) * 80.0),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                ..Default::default()
            },
        )?;
        Ok(())
    }

    pub fn draw_description(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        let font = state.data.as_ref().unwrap().get_font();
        let mut next_wave = "".to_string();
        if let WaveStatus::Waiting(time) = state.waves.status {
            next_wave = format!(", Next wave in {}", time / 60);
        }
        let mut desc = Text::new(
            ctx,
            &format!(
                "Lives: {}, Gold: {}{}",
                state.player().hp,
                state.player().gold,
                next_wave
            ),
            font,
        )?;
        desc.set_filter(graphics::FilterMode::Nearest);

        graphics::draw_ex(
            ctx,
            &desc,
            graphics::DrawParam {
                // src: src,
                dest: Point2::new(10.0, 560.0),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.0, 0.0),
                scale: Point2::new(1.0, 1.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        return Ok(());
    }

    pub fn draw(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        Gui::draw_cards(state, ctx)?;
        match state.gui.cursor_state {
            CursorMode::Map { x, y, card, .. } => {
                state
                    .gui
                    .draw_map_cursor(x, y, &state.data.as_ref().unwrap(), ctx)?;
                Gui::draw_effect_preview(state, x, y, card, ctx)?;
            }
            CursorMode::Hand(slot) => {
                state
                    .gui
                    .draw_cards_cursor(slot, &state.data.as_ref().unwrap(), ctx)?;
            }
        }
        Gui::draw_description(state, ctx)?;
        Ok(())
    }

    pub fn key_down(state: &mut PlayingState, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => Gui::move_cursor(state, 0, -1),
            Keycode::Down => Gui::move_cursor(state, 0, 1),
            Keycode::Left => Gui::move_cursor(state, -1, 0),
            Keycode::Right => Gui::move_cursor(state, 1, 0),
            Keycode::Escape => Gui::chancel(state),
            Keycode::Space => match state.gui.cursor_state {
                CursorMode::Map { x, y, slot, card } => {
                    Gui::event_activate(state, x, y, slot, card)
                }
                CursorMode::Hand(slot) => Gui::event_select(state, slot),
            },
            _ => {}
        }
    }

    fn event_activate(state: &mut PlayingState, x: usize, y: usize, slot: usize, card: CardType) {
        if card.is_applicable(state, x, y) {
            card.activate(state, x, y);
            state.player_mut().deck.card_used(slot);
        }
    }

    fn event_select(state: &mut PlayingState, slot: usize) {
        let card = state.player().deck.hand[slot].clone();
        card.select(state, slot);
    }
}
