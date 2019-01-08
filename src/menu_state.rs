use crate::assets::Data;
use crate::event_handler::{self, StateTransition};
use crate::playing_state::PlayingState;
use crate::utils::add_mod;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Color, Point2, Text};
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
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;

        let font = self.data.as_ref().unwrap().get_font();
        for (i, item) in self.options.iter().enumerate() {
            let mut desc = Text::new(ctx, &item.get_text(), font)?;
            desc.set_filter(graphics::FilterMode::Nearest);
            let mut color = Color::new(1.0, 1.0, 1.0, 1.0);
            if i == self.option_selected {
                color = Color::new(1.0, 1.0, 0.0, 1.0);
            }

            graphics::draw_ex(
                ctx,
                &desc,
                graphics::DrawParam {
                    // src: src,
                    dest: Point2::new(300.0, 100.0 + 40.0 * i as f32),
                    //rotation: self.zoomlevel,
                    offset: Point2::new(0.0, 0.0),
                    scale: Point2::new(1.0, 1.0),
                    color: Some(color),
                    // shear: shear,
                    ..Default::default()
                },
            )?;
        }
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
            Keycode::Up => {
                self.option_selected = add_mod(self.option_selected, -1, self.options.len())
            }
            Keycode::Down => {
                self.option_selected = add_mod(self.option_selected, 1, self.options.len())
            }
            Keycode::Space => match self.options[self.option_selected] {
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
