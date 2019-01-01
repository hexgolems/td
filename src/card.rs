use crate::assets::ImgID;
use crate::game_state::GameState;
use crate::gui::CursorMode;
use crate::map::GameMap;
use crate::towers::{Tower, TowerType};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    Build(TowerType),
    SellTower,
    DamageEnemy,
}
use self::CardType::*;

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::Build(TowerType::Cannon) => ImgID::Cannon,
            CardType::Build(TowerType::Archers) => ImgID::Archers,
            CardType::SellTower => ImgID::SellTower,
            CardType::DamageEnemy => ImgID::DamageEnemy,
        }
    }
    pub fn get_preview_image_id(&self) -> ImgID {
        return self.get_image_id();
    }

    pub fn select(&self, state: &mut GameState, slot: usize) {
        match self {
            CardType::Empty => {}
            CardType::Build(_) => state.gui.set_cursor_card_effect(slot, self),
            CardType::SellTower => state.gui.set_cursor_card_effect(slot, self),
            CardType::DamageEnemy => state.gui.set_cursor_card_effect(slot, self),
        }
    }
    pub fn is_applicable(&self, state: &GameState, x: usize, y: usize) -> bool {
        match self {
            CardType::Empty => return false,
            CardType::Build(_) => {
                return state.map.is_buildable(x, y) && !state.towers.has_building(x, y);
            }
            CardType::SellTower => return state.towers.has_building(x, y),
            CardType::DamageEnemy => {
                return state
                    .enemies
                    .in_range(GameMap::tile_center(x, y), 80.0)
                    .len()
                    > 0;
            }
        }
    }

    pub fn activate(&self, state: &mut GameState, x: usize, y: usize) {
        match self {
            CardType::Empty => {}
            CardType::Build(t) => {
                state.towers.spawn(Tower::new(*t, (x, y)));
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::SellTower => {
                state.towers.remove_tower(x, y);
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::DamageEnemy => {
                for e in state.enemies.in_range(GameMap::tile_center(x, y), 80.0) {
                    state.enemies.damage(e, 150);
                }
                state.gui.set_cursor(CursorMode::Hand(0));
            }
        }
    }
}

pub struct CardDeck {
    pub hand: Vec<CardType>,
    pub deck: Vec<CardType>,
    pub discard: Vec<CardType>,
}

impl CardDeck {
    pub fn new() -> Self {
        let hand = vec![];
        let deck = vec![
            Build(TowerType::Cannon),
            Build(TowerType::Archers),
            DamageEnemy,
            SellTower,
        ];
        let discard = vec![];
        Self {
            hand,
            deck,
            discard,
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.as_mut_slice().shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(card) = self.draw_one() {
                self.hand.push(card);
            }
        }
    }

    pub fn draw_one(&mut self) -> Option<CardType> {
        if self.deck.is_empty() {
            self.deck.append(&mut self.discard);
            self.shuffle()
        }
        return self.deck.pop();
    }
}
