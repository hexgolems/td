use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Point2, Text};
use ggez::{Context, GameResult};

use crate::assets::{FontID, ImgID};
use crate::card::CardType;
use crate::game_state::GameState;
use crate::overlay_state::{OverlayState, StateTransition};
use crate::towers::TowerType;
use crate::utils::add_mod;

pub struct ShopOverlay {
    card_used: usize,
    cur_selected: usize,
}

impl ShopOverlay {
    pub fn new(card_used: usize) -> Self {
        return Self {
            card_used,
            cur_selected: 0,
        };
    }

    fn get_available_cards(&self, _state: &GameState) -> Vec<CardType> {
        return vec![
            CardType::Build(TowerType::Archer),
            CardType::Build(TowerType::Cannon),
            CardType::Shop,
            CardType::SellTower,
            CardType::DamageEnemy,
        ];
    }

    fn draw_available_cards(&self, state: &GameState, ctx: &mut Context) -> GameResult<()> {
        for (i, card) in self.get_available_cards(state).iter().enumerate() {
            graphics::draw_ex(
                ctx,
                state.data.get_i(&card.get_image_id()),
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(100.0, 40.0 + (i as f32) * 80.0),
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

    fn draw_cursor(&self, state: &GameState, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(
            ctx,
            state.data.get_i(&ImgID::Cursor),
            graphics::DrawParam {
                // src: src,
                dest: Point2::new(100.0, 40.0 + (self.cur_selected as f32) * 80.0),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(4.0, 4.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        return Ok(());
    }

    fn draw_selected(&self, state: &GameState, ctx: &mut Context) -> GameResult<()> {
        let card = self.get_available_cards(state)[self.cur_selected];
        graphics::draw_ex(
            ctx,
            state.data.get_i(&card.get_image_id()),
            graphics::DrawParam {
                // src: src,
                dest: Point2::new(300.0, 40.0),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.0, 0.0),
                scale: Point2::new(8.0, 8.0),
                // shear: shear,
                ..Default::default()
            },
        )?;
        let font = state.data.get_font();
        let (_, txts) = font.get_wrap(card.get_description(), 200);
        for (i, txt) in txts.iter().enumerate() {
            let mut desc = Text::new(ctx, txt, font)?;
            desc.set_filter(graphics::FilterMode::Nearest);

            graphics::draw_ex(
                ctx,
                &desc,
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(300.0, 200.0 + (i as f32) * 30.0),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.0, 0.0),
                    scale: Point2::new(1.0, 1.0),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
        return Ok(());
    }
}

impl OverlayState for ShopOverlay {
    fn update(&mut self, _state: &mut GameState) -> GameResult<StateTransition> {
        return Ok(StateTransition::Stay);
    }

    fn draw(&self, state: &GameState, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.draw_available_cards(state, ctx)?;
        self.draw_cursor(state, ctx)?;
        self.draw_selected(state, ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        state: &mut GameState,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) -> StateTransition {
        match keycode {
            Keycode::Up => {
                self.cur_selected =
                    add_mod(self.cur_selected, -1, self.get_available_cards(state).len())
            }
            Keycode::Down => {
                self.cur_selected =
                    add_mod(self.cur_selected, 1, self.get_available_cards(state).len())
            }
            //Keycode::Left => Gui::move_cursor(state, -1, 0),
            //Keycode::Right => Gui::move_cursor(state, 1, 0),
            Keycode::Escape => {
                state.deck.card_used(self.card_used);
                return StateTransition::Return;
            }
            //Keycode::Space => match state.gui.cursor_state {
            //    CursorMode::Map { x, y, slot, card } => {
            //        Gui::event_activate(state, x, y, slot, card)
            //    }
            //    CursorMode::Hand(slot) => Gui::event_select(state, slot),
            //},
            _ => {}
        }
        return StateTransition::Stay;
    }
}
