use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
//use ggez::timer;
use ggez::{Context, GameResult};

use crate::assets::Imgs;
use crate::card::CardDeck;
use crate::enemies::Enemies;
use crate::gui::Gui;
use crate::map::GameMap;
use crate::projectiles::Projectiles;
use crate::towers::Towers;
use crate::wave::Waves;

pub struct GameState {
    pub imgs: Imgs,
    pub map: GameMap,
    pub enemies: Enemies,
    pub towers: Towers,
    pub waves: Waves,
    pub gui: Gui,
    pub deck: CardDeck,
    pub hp: usize,
    pub projectiles: Projectiles,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut imgs = Imgs::new();
        imgs.init(ctx).expect("couldn't load resources");
        let map = GameMap::new();
        let enemies = Enemies::new();
        let towers = Towers::new();
        let waves = Waves::new();
        let gui = Gui::new();
        let mut deck = CardDeck::new();
        deck.shuffle();
        deck.draw(4);
        let hp = 1000000;
        let projectiles = Projectiles::new();

        let s = Self {
            imgs,
            map,
            enemies,
            towers,
            waves,
            gui,
            deck,
            hp,
            projectiles,
        };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        self.map.draw(&self.imgs, ctx)?;
        self.enemies.draw(&self.imgs, ctx)?;
        self.towers.draw(&self.imgs, ctx)?;
        //self.gui.draw(&self.imgs, ctx)?;
        Gui::draw(self, ctx)?;
        self.projectiles.draw(&self.imgs, ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        Gui::key_down(self, keycode, keymod, repeat);
    }
}
