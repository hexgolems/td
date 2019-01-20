use ggez::event::{KeyCode, Mod};
use ggez::{Context, GameResult};

use crate::event_handler::StateTransition;
use crate::playing_state::PlayingState;

pub trait OverlayState {
    fn update(&mut self, state: &mut PlayingState) -> GameResult<(StateTransition)>;
    fn draw(&self, state: &PlayingState, ctx: &mut Context) -> GameResult<()>;
    fn key_down_event(
        &mut self,
        state: &mut PlayingState,
        keycode: KeyCode,
        keymod: Mod,
        repeat: bool,
    ) -> StateTransition;
}
