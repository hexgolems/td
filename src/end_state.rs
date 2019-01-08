use crate::assets::Data;
use crate::event_handler::{self, StateTransition};
use crate::menu_state::MenuState;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Color, Point2, Text};
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
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;

        let font = self.data.as_ref().unwrap().get_font();
        let text = if self.victory {
            "Congratz, You Won!"
        } else {
            "Ouch, You Failed!"
        };
        let mut desc = Text::new(ctx, text, font)?;
        desc.set_filter(graphics::FilterMode::Nearest);
        let color = if self.victory {
            Color::new(0.2, 1.0, 0.2, 1.0)
        } else {
            Color::new(1.0, 0.2, 0.2, 1.0)
        };

        graphics::draw_ex(
            ctx,
            &desc,
            graphics::DrawParam {
                // src: src,
                dest: Point2::new(300.0, 100.0),
                //rotation: self.zoomlevel,
                offset: Point2::new(0.0, 0.0),
                scale: Point2::new(1.0, 1.0),
                color: Some(color),
                // shear: shear,
                ..Default::default()
            },
        )?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) -> StateTransition {
        match keycode {
            Keycode::Space => return StateTransition::Next(Box::new(MenuState::new())),
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
