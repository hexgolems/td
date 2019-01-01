use ggez::event::{Keycode, Mod};
use ggez::{Context, GameResult};

use crate::game_state::GameState;

pub trait OverlayState {
    fn update(&mut self, state: &mut GameState) -> GameResult<(StateTransition)>;
    fn draw(&self, state: &GameState, ctx: &mut Context) -> GameResult<()>;
    fn key_down_event(
        &mut self,
        state: &mut GameState,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    ) -> StateTransition;
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum StateTransition {
    Stay,
    Return,
}

