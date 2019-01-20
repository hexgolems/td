use crate::algebra::{Point, Vector};
use crate::assets::Data;
use crate::event_handler::{self, StateTransition};
use crate::playing_state::PlayingState;
use crate::utils::add_mod;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Scale, Text, TextFragment};
use ggez::{Context, GameResult};

enum MenuItem {
    Level(String),
    Exit,
}

impl MenuItem {
    fn get_text(&self) -> String {
        match self {
            MenuItem::Level(a) => format!("Play Level {}", &a),
            MenuItem::Exit => "Exit".to_string(),
        }
    }
}

pub struct MenuState {
    option_selected: usize,
    options: Vec<MenuItem>,
    data: Option<Data>,
}

impl MenuState {
    pub fn new() -> Self {
        let options = vec![MenuItem::Level("Test".to_string()), MenuItem::Exit];
        return Self {
            option_selected: 0,
            options,
            data: None,
        };
    }
}

impl event_handler::GameState for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<StateTransition> {
        return Ok(StateTransition::Stay);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.1, 0.2, 0.2, 0.0));
        //graphics::set_color(ctx, graphics::WHITE)?;

        let font = self.data.as_ref().unwrap().get_font();
        for (i, item) in self.options.iter().enumerate() {
            let tf = TextFragment::new(item.get_text());
            let mut desc = Text::new(tf);
            desc.set_font(*font, Scale::uniform(24.0));
            //desc.set_filter(graphics::FilterMode::Nearest);
            let mut color = Color::new(1.0, 1.0, 1.0, 1.0);
            if i == self.option_selected {
                color = Color::new(1.0, 1.0, 0.0, 1.0);
            }

            graphics::draw(
                ctx,
                &desc,
                graphics::DrawParam::default()
                    .dest(Point::new(300.0, 100.0 + 40.0 * i as f32))
                    .offset(Point::new(0.0, 0.0))
                    .scale(Vector::new(1.0, 1.0))
                    .color(color),
            )?;
        }
        graphics::present(ctx)?;
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
            KeyCode::Up => {
                self.option_selected = add_mod(self.option_selected, -1, self.options.len())
            }
            KeyCode::Down => {
                self.option_selected = add_mod(self.option_selected, 1, self.options.len())
            }
            KeyCode::Space => match self.options[self.option_selected] {
                MenuItem::Level(_) => return StateTransition::Next(Box::new(PlayingState::new())),
                MenuItem::Exit => return StateTransition::Exit,
            },
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
