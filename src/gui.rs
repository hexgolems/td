use crate::algebra::{Point, Vector};
use crate::assets::{Data, ImgID};
use crate::camera::Camera;
use crate::card::CardType;
use crate::map::GameMap;
use crate::playing_state::PlayingState;
use crate::utils::{self, add_mod};
use crate::wave::WaveStatus;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CursorMode {
    Map {
        x: usize,
        y: usize,
        card: CardType,
        slot: usize,
    },
    Actions(usize),
}

use self::CursorMode::*;

pub struct Gui {
    cursor_state: CursorMode,
    camera: Camera,
}

impl Gui {
    pub fn new() -> Self {
        let cursor_state = CursorMode::Actions(0);
        let camera = Camera::new();
        return Self {
            cursor_state,
            camera,
        };
    }

    pub fn set_cursor(&mut self, c: CursorMode) {
        self.cursor_state = c;
    }

    pub fn chancel(state: &mut PlayingState) {
        state.gui.set_cursor(CursorMode::Actions(0));
    }

    pub fn move_cursor(state: &mut PlayingState, ix: isize, iy: isize) {
        let len = state.player().deck.hand.len().clone() + state.player().deck.actions.len();
        match state.gui.cursor_state {
            Map {
                ref mut x,
                ref mut y,
                ..
            } => {
                *y = add_mod(*y, iy, state.map.ysize);
                *x = add_mod(*x, ix, state.map.xsize);
            }
            Actions(ref mut slot) => {
                if len > 0 {
                    *slot = add_mod(*slot, ix, len);
                }
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

    pub fn cam(&self) -> &Camera {
        return &self.camera;
    }

    pub fn cam_mut(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    pub fn tick(state: &mut PlayingState) {
        Camera::tick(state);
    }

    fn draw_map_cursor(
        &self,
        x: usize,
        y: usize,
        data: &Data,
        ctx: &mut Context,
    ) -> GameResult<()> {
        graphics::draw(
            ctx,
            data.get_i(&ImgID::CursorMap),
            graphics::DrawParam::default()
                .dest(self.camera.world_pos(GameMap::tile_pos(x, y)))
                .offset(Point::new(1.0 / 71.0, 1.0 / 79.0))
                .scale(Vector::new(1.0, 1.0)),
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
        graphics::draw(
            ctx,
            state
                .data
                .as_ref()
                .unwrap()
                .get_i(&card.get_preview_image_id()),
            graphics::DrawParam::default()
                .dest(state.gui.camera.world_pos(GameMap::tile_center(x, y)))
                .offset(Point::new(0.5, 0.5))
                .scale(Vector::new(4.0, 4.0))
                .color(color),
        )?;
        Ok(())
    }

    fn draw_cards(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in state.player().deck.hand.iter().enumerate() {
            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&ImgID::Card),
                graphics::DrawParam::default()
                    .dest(Point::new(50.0 + (i as f32) * 80.0, 550.0))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;

            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&card.get_image_id()),
                graphics::DrawParam::default()
                    .dest(Point::new(50.0 + (i as f32) * 80.0, 550.0))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;

            let gold = card.activation_cost_gold(state);
            if gold > 0 {
                let desc = utils::text(state.data.as_ref().unwrap(), &format!("{}", gold));
                let color = graphics::Color::new(0.8672, 0.6392, 0.2117, 1.0);
                graphics::draw(
                    ctx,
                    &desc,
                    graphics::DrawParam::default()
                        .dest(Point::new(50.0 + (i as f32) * 80.0, 550.0))
                        .offset(Point::new(1.0, 1.0))
                        .scale(Vector::new(0.3, 0.3))
                        .color(color),
                )?;
            }
            let mana = card.activation_cost_mana(state);
            if mana > 0 {
                let desc = utils::text(state.data.as_ref().unwrap(), &format!("{}", mana));
                let color = graphics::Color::new(0.2, 0.2, 1.0, 1.0);
                graphics::draw(
                    ctx,
                    &desc,
                    graphics::DrawParam::default()
                        .dest(Point::new(20.0 + (i as f32) * 80.0, 550.0))
                        .offset(Point::new(1.0, 1.0))
                        .scale(Vector::new(0.3, 0.3))
                        .color(color),
                )?;
            }
        }
        Ok(())
    }

    fn draw_card_info(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        if let Actions(id) = state.gui.cursor_state {
            if let Some(card) = state.player().deck.get_selected_card(id) {
                graphics::draw(
                    ctx,
                    state.data.as_ref().unwrap().get_i(&card.get_image_id()),
                    graphics::DrawParam::default()
                        .dest(Point::new(600.0, 40.0))
                        .offset(Point::new(0.0, 0.0))
                        .scale(Vector::new(8.0, 8.0)),
                )?;
                let mut desc = utils::text(state.data.as_ref().unwrap(), &card.get_description());
                desc.set_bounds(Point::new(600.0, 400.0), graphics::Align::Left);
                graphics::draw(
                    ctx,
                    &desc,
                    graphics::DrawParam::default()
                        .dest(Point::new(600.0, 200.0))
                        .offset(Point::new(0.0, 0.0))
                        .scale(Vector::new(0.3, 0.3)),
                )?;
            }
        }
        return Ok(());
    }

    fn draw_actions(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in state.player().deck.actions.iter().rev().enumerate() {
            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&ImgID::Card),
                graphics::DrawParam::default()
                    .dest(Point::new(750.0 - (i as f32) * 80.0, 550.0))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;

            graphics::draw(
                ctx,
                state.data.as_ref().unwrap().get_i(&card.get_image_id()),
                graphics::DrawParam::default()
                    .dest(Point::new(750.0 - (i as f32) * 80.0, 550.0))
                    .offset(Point::new(0.5, 0.5))
                    .scale(Vector::new(4.0, 4.0)),
            )?;

            let gold = card.activation_cost_gold(state);
            if gold > 0 {
                let desc = utils::text(state.data.as_ref().unwrap(), &format!("{}", gold));

                graphics::draw(
                    ctx,
                    &desc,
                    graphics::DrawParam::default()
                        .dest(Point::new(720.0 - (i as f32) * 80.0, 550.0))
                        .offset(Point::new(1.0, 1.0))
                        .scale(Vector::new(0.3, 0.3)),
                )?;
            }
        }
        Ok(())
    }

    fn draw_cards_cursor(
        state: &PlayingState,
        slot: usize,
        data: &Data,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let hand_len = state.player().deck.hand.len();
        let actions_len = state.player().deck.actions.len();
        let pos = if slot < hand_len {
            Point::new(50.0 + (slot as f32) * 80.0, 550.0)
        } else {
            Point::new(
                750.0 - ((actions_len - 1 - (slot - hand_len)) as f32) * 80.0,
                550.0,
            )
        };
        graphics::draw(
            ctx,
            data.get_i(&ImgID::Cursor),
            graphics::DrawParam::default()
                .dest(pos)
                .offset(Point::new(0.5, 0.5))
                .scale(Vector::new(4.0, 4.0)),
        )?;
        Ok(())
    }

    pub fn draw_description(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        let mut next_wave = "".to_string();
        if let WaveStatus::Waiting(time) = state.waves.status {
            next_wave = format!(", Next wave in {}", time / 60);
        }
        let desc = utils::text(
            state.data.as_ref().unwrap(),
            &format!(
                "Lives: {}, Gold: {} Mana: {}{}",
                state.player().hp,
                state.player().gold,
                state.player().mana as u64,
                next_wave
            ),
        );

        graphics::draw(
            ctx,
            &desc,
            graphics::DrawParam::default()
                .dest(Point::new(10.0, 2.0))
                .offset(Point::new(0.0, 0.0))
                .scale(Vector::new(0.3, 0.3)),
        )?;
        return Ok(());
    }

    pub fn draw_tower_info(
        state: &PlayingState,
        x: usize,
        y: usize,
        ctx: &mut Context,
    ) -> GameResult<()> {
        let mut info = "".to_string();
        if let Some(stats) = state.towers.stats_at(x, y) {
            info += &stats.info();
            info += "\n";
        }
        if let Some(buffs) = state.towers.buffs_at(x, y) {
            for buff in buffs.iter() {
                info += &buff.info();
                info += "\n";
            }
        }
        let desc = utils::text(state.data.as_ref().unwrap(), &info);
        graphics::draw(
            ctx,
            &desc,
            graphics::DrawParam::default()
                .dest(Point::new(600.0, 50.0))
                .offset(Point::new(0.0, 0.0))
                .scale(Vector::new(0.3, 0.3)),
        )?;
        return Ok(());
    }

    pub fn draw(state: &PlayingState, ctx: &mut Context) -> GameResult<()> {
        Gui::draw_cards(state, ctx)?;
        Gui::draw_actions(state, ctx)?;

        match state.gui.cursor_state {
            CursorMode::Map { x, y, card, .. } => {
                state
                    .gui
                    .draw_map_cursor(x, y, &state.data.as_ref().unwrap(), ctx)?;
                Gui::draw_effect_preview(state, x, y, card, ctx)?;
                Gui::draw_tower_info(state, x, y, ctx)?;
            }
            CursorMode::Actions(slot) => {
                Gui::draw_cards_cursor(state, slot, &state.data.as_ref().unwrap(), ctx)?;
                Gui::draw_card_info(state, ctx)?;
            }
        }
        Gui::draw_description(state, ctx)?;
        Ok(())
    }

    pub fn key_down(state: &mut PlayingState, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => Gui::move_cursor(state, 0, -1),
            KeyCode::Down => Gui::move_cursor(state, 0, 1),
            KeyCode::Left => Gui::move_cursor(state, -1, 0),
            KeyCode::Right => Gui::move_cursor(state, 1, 0),
            KeyCode::Escape => Gui::chancel(state),
            KeyCode::Space => match state.gui.cursor_state {
                CursorMode::Map { x, y, slot, card } => {
                    Gui::event_activate(state, x, y, slot, card)
                }
                CursorMode::Actions(slot) => Gui::event_select(state, slot),
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
        if let Some(card) = state.player().deck.get_selected_card(slot) {
            if card.is_selectable(state, slot) {
                card.clone().select(state, slot);
            }
        }
    }
}
