use ggez::event::{KeyCode, Mod};
use ggez::graphics;
//use ggez::timer;
use ggez::{Context, GameResult};

use crate::assets::Data;
use crate::background::Background;
use crate::effects::Effects;
use crate::end_state::EndState;
use crate::enemies::Enemies;
use crate::event_handler::{self, StateTransition};
use crate::gui::Gui;
use crate::map::GameMap;
use crate::overlay_state::OverlayState;
use crate::player::Player;
use crate::projectiles::Projectiles;
use crate::towers::Towers;
use crate::wave::{WaveStatus, Waves};
use std::collections::HashMap;

pub struct PlayingState {
    pub me: usize,
    pub data: Option<Data>,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub background: Background,
    pub waves: Waves,
    pub gui: Gui,
    pub players: HashMap<usize, Player>,
    pub projectiles: Projectiles,
    pub effects: Effects,
    pub overlay_state: Option<Box<OverlayState>>,
}

impl PlayingState {
    pub fn new() -> Self {
        let data = None;
        let map = GameMap::new();
        let enemies = Enemies::new();
        let towers = Towers::new();
        let waves = Waves::new();
        let gui = Gui::new();
        let projectiles = Projectiles::new();
        let mut players = HashMap::new();
        let me = 42;
        let player = Player::new(me);
        let effects = Effects::new();
        let background = Background::new();
        players.insert(me, player);

        return Self {
            me,
            data,
            map,
            enemies,
            towers,
            waves,
            gui,
            projectiles,
            overlay_state: None,
            players,
            effects,
            background,
        };
    }

    pub fn player_mut(&mut self) -> &mut Player {
        self.players.get_mut(&self.me).unwrap()
    }

    pub fn player(&self) -> &Player {
        self.players.get(&self.me).unwrap()
    }
}

impl event_handler::GameState for PlayingState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<event_handler::StateTransition> {
        if let Some(mut overlay) = self.overlay_state.take() {
            match overlay.update(self)? {
                StateTransition::Stay => self.overlay_state = Some(overlay),
                StateTransition::Return => {}
                _ => unreachable!(),
            }
            return Ok(event_handler::StateTransition::Stay);
        }
        const _DESIRED_FPS: u32 = 60;
        if self.player().hp <= 0 {
            return Ok(event_handler::StateTransition::Next(Box::new(
                EndState::failed(),
            )));
        }
        if self.waves.status == WaveStatus::LevelFinished {
            return Ok(event_handler::StateTransition::Next(Box::new(
                EndState::win(),
            )));
        }
        Waves::tick(self);
        Enemies::tick(self);
        Towers::tick(self);
        Projectiles::tick(self);
        self.background.tick();
        self.effects.tick();
        if self.waves.status == WaveStatus::WaveFinished {
            self.waves.status = WaveStatus::Waiting(5 * 60);
        }
        if self.waves.status == WaveStatus::Ready {
            self.player_mut().deck.discard_all();
            self.player_mut().deck.draw(5);
            self.waves.status = WaveStatus::Ongoing;
        }
        return Ok(event_handler::StateTransition::Stay);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(overlay) = self.overlay_state.take() {
            overlay.draw(self, ctx)?;
            self.overlay_state = Some(overlay);
            return Ok(());
        }
        graphics::clear(ctx, Color::new(1.0, 1.0, 1.0, 1.0));
        graphics::set_color(ctx, graphics::WHITE)?;
        Background::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        GameMap::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        Enemies::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        Towers::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        Projectiles::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        Effects::draw(&self, &self.data.as_ref().unwrap(), ctx)?;
        Gui::draw(self, ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: Mod,
        repeat: bool,
    ) -> event_handler::StateTransition {
        if let Some(mut overlay) = self.overlay_state.take() {
            match overlay.key_down_event(self, keycode, keymod, repeat) {
                StateTransition::Stay => self.overlay_state = Some(overlay),
                StateTransition::Return => {}
                _ => unreachable!(),
            }
            return event_handler::StateTransition::Stay;
        }
        Gui::key_down(self, keycode, keymod, repeat);
        return event_handler::StateTransition::Stay;
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }
    fn take_data(&mut self) -> Data {
        return self.data.take().unwrap();
    }
}
