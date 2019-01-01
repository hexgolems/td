use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
//use ggez::timer;
use ggez::{Context, GameResult};

use crate::assets::Data;
use crate::card::CardDeck;
use crate::enemies::Enemies;
use crate::gui::Gui;
use crate::map::GameMap;
use crate::overlay_state::{OverlayState, StateTransition};
use crate::projectiles::Projectiles;
use crate::towers::Towers;
use crate::wave::Waves;

pub struct GameState {
    pub data: Data,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub waves: Waves,
    pub gui: Gui,
    pub deck: CardDeck,
    pub hp: usize,
    pub projectiles: Projectiles,
    pub overlay_state: Option<Box<OverlayState>>,
    pub gold: usize,
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
        let mut deck = CardDeck::new();
        deck.shuffle();
        deck.draw(5);
        let hp = 1000000;
        let gold = 500;
        let projectiles = Projectiles::new();

        let s = Self {
            data,
            map,
            enemies,
            towers,
            waves,
            gui,
            deck,
            hp,
            gold,
            projectiles,
            overlay_state: None,
        };
        Ok(s)
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
        assert!(self.hp > 0, "0xDEAD");
        //while timer::check_update_time(ctx, DESIRED_FPS) {
        Waves::tick(self);
        Enemies::tick(self);
        Towers::tick(self);
        Projectiles::tick(self);
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
