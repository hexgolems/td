use crate::algebra::{Point, Vector};
use crate::assets::Data;
use crate::event_handler::{self, StateTransition};
use crate::menu_state::MenuState;
use crate::utils;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Scale, Text};
use ggez::{Context, GameResult};

pub struct EndState {
    victory: bool,
    data: Option<Data>,
}

impl EndState {
    fn new(victory: bool) -> Self {
        return Self {
            victory,
            data: None,
        };
    }
    pub fn failed() -> Self {
        return EndState::new(false);
    }
    pub fn win() -> Self {
        return EndState::new(true);
    }
}

impl event_handler::GameState for EndState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<StateTransition> {
        return Ok(StateTransition::Stay);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.1, 0.2, 0.4, 1.0));
        let text = if self.victory {
            "Congratz, You Won!"
        } else {
            "Ouch, You Failed!"
        };
        let desc = utils::text(self.data.as_ref().unwrap(), &text);
        let color = if self.victory {
            Color::new(0.2, 1.0, 0.2, 1.0)
        } else {
            Color::new(1.0, 0.2, 0.2, 1.0)
        };

        graphics::draw(
            ctx,
            &desc,
            graphics::DrawParam::default()
                .dest(Point::new(300.0, 100.0))
                .offset(Point::new(0.0, 0.0))
                .scale(Vector::new(0.3, 0.3))
                .color(color),
        )?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) -> StateTransition {
        match keycode {
            KeyCode::Space => return StateTransition::Next(Box::new(MenuState::new())),
            _ => {}
        }
        return StateTransition::Stay;
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }
    fn take_data(&mut self) -> Data {
        return self.data.take().unwrap();
    }
}
