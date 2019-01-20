use crate::algebra::{Point, Vector};
use crate::assets::Data;
use crate::event_handler::{self, StateTransition};
use crate::menu_state::MenuState;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Scale, Text, TextFragment};
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
        graphics::clear(ctx, Color::new(1.0, 1.0, 1.0, 1.0));
        //graphics::set_color(ctx, graphics::WHITE)?;

        let font = self.data.as_ref().unwrap().get_font();
        let text = if self.victory {
            "Congratz, You Won!"
        } else {
            "Ouch, You Failed!"
        };
        let tf = TextFragment::new(text);
        let mut desc = Text::new(tf);
        desc.set_font(*font, Scale::uniform(1.0));
        //desc.set_filter(graphics::FilterMode::Nearest);
        let color = if self.victory {
            Color::new(0.2, 1.0, 0.2, 1.0)
        } else {
            Color::new(1.0, 0.2, 0.2, 1.0)
        };

        graphics::draw(
            ctx,
            &desc,
            graphics::DrawParam {
                // src: src,
                dest: Point::new(300.0, 100.0),
                //rotation: self.zoomlevel,
                offset: Point::new(0.0, 0.0),
                scale: Vector::new(1.0, 1.0),
                color: color,
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
