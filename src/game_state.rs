use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
//use ggez::timer;
use ggez::{Context, GameResult};

use crate::assets::Data;
use crate::enemies::Enemies;
use crate::gui::Gui;
use crate::map::GameMap;
use crate::overlay_state::{OverlayState, StateTransition};
use crate::player::Player;
use crate::projectiles::Projectiles;
use crate::towers::Towers;
use crate::wave::{WaveStatus, Waves};
use std::collections::HashMap;

pub struct GameState {
    pub me: usize,
    pub data: Data,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub waves: Waves,
    pub gui: Gui,
    pub players: HashMap<usize, Player>,
    pub projectiles: Projectiles,
    pub overlay_state: Option<Box<OverlayState>>,
}

pub struct GameStateShared {
    pub player: Player,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub projectiles: Projectiles,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut data = Data::new();
        data.init(ctx).expect("couldn't load resources");
        let map = GameMap::new();
        let enemies = Enemies::new();
        let towers = Towers::new();
        let waves = Waves::new();
        let gui = Gui::new();
        let projectiles = Projectiles::new();
        let mut players = HashMap::new();
        let me = 42;
        let player = Player::new(me);
        players.insert(me, player);

        let s = Self {
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
        };
        Ok(s)
    }

    pub fn player_mut(&mut self) -> &mut Player {
        self.players.get_mut(&self.me).unwrap()
    }

    pub fn player(&self) -> &Player {
        self.players.get(&self.me).unwrap()
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Some(mut overlay) = self.overlay_state.take() {
            match overlay.update(self)? {
                StateTransition::Stay => self.overlay_state = Some(overlay),
                StateTransition::Return => {}
            }
            return Ok(());
        }
        const _DESIRED_FPS: u32 = 60;
        assert!(self.player().hp > 0, "0xDEAD");
        //while timer::check_update_time(ctx, DESIRED_FPS) {
        Waves::tick(self);
        Enemies::tick(self);
        Towers::tick(self);
        Projectiles::tick(self);
        //Wait for
        if self.waves.status == WaveStatus::Finished {
            self.waves.status = WaveStatus::Waiting(5 * 60);
        }
        if self.waves.status == WaveStatus::Ready {
            self.player_mut().deck.discard_all();
            self.player_mut().deck.draw(5);
            self.waves.status = WaveStatus::Ongoing;
        }

        //}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(overlay) = self.overlay_state.take() {
            overlay.draw(self, ctx)?;
            self.overlay_state = Some(overlay);
            return Ok(());
        }
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.map.draw(&self.data, ctx)?;
        self.enemies.draw(&self.data, ctx)?;
        self.towers.draw(&self.data, ctx)?;
        //self.gui.draw(&self.data, ctx)?;
        Gui::draw(self, ctx)?;
        self.projectiles.draw(&self.data, ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        if let Some(mut overlay) = self.overlay_state.take() {
            match overlay.key_down_event(self, keycode, keymod, repeat) {
                StateTransition::Stay => self.overlay_state = Some(overlay),
                StateTransition::Return => {}
            }
            return;
        }
        Gui::key_down(self, keycode, keymod, repeat);
    }
}
