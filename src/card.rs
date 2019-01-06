use crate::assets::ImgID;
use crate::buffs::BuffType;
use crate::game_state::GameState;
use crate::gui::CursorMode;
use crate::map::GameMap;
use crate::shop_overlay::ShopOverlay;
use crate::towers::Tower;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    Tower,
    SellTower,
    DamageEnemy,
    Shop,
    Coin(usize),
    Take2,
    Buff(BuffType),
}
use self::CardType::*;

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::Tower => ImgID::Archer,
            CardType::SellTower => ImgID::SellTower,
            CardType::DamageEnemy => ImgID::DamageEnemy,
            CardType::Shop => ImgID::Shop,
            CardType::Coin(a) => ImgID::Coin(*a),
            CardType::Take2 => ImgID::Take2,
            CardType::Buff(BuffType::Freeze) => ImgID::Freeze,
        }
    }

    pub fn get_preview_image_id(&self) -> ImgID {
        return self.get_image_id();
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            CardType::Empty => "",
            CardType::Tower => "Build a tower",
            CardType::SellTower => "Allows you to destroy a tower",
            CardType::DamageEnemy => "Damages all enemies in a given range",
            CardType::Shop => "Allows you to buy one card",
            CardType::Coin(1) => "Produces 10 Gold",
            CardType::Coin(2) => "Produces 100 Gold",
            CardType::Coin(3) => "Produces 1000 Gold",
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => "Draw 2 more cards",
            CardType::Buff(BuffType::Freeze) => "Adds freeze buff to slow down enemies",
        }
    }

    pub fn activation_cost(&self, state: &GameState) -> usize {
        match self {
            CardType::Empty => 0,
            CardType::Tower => state.towers.base_stats.price,
            CardType::SellTower => 0,
            CardType::DamageEnemy => 150,
            CardType::Shop => 0,
            CardType::Coin(1) => 0,
            CardType::Coin(2) => 0,
            CardType::Coin(3) => 0,
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => 10,
            CardType::Buff(BuffType::Freeze) => 10,
        }
    }

    pub fn aquisition_cost(&self, _state: &GameState) -> usize {
        match self {
            CardType::Empty => 0,
            CardType::Tower => 60,
            CardType::SellTower => 10,
            CardType::DamageEnemy => 100,
            CardType::Shop => 100,
            CardType::Coin(1) => 30,
            CardType::Coin(2) => 300,
            CardType::Coin(3) => 3000,
            CardType::Coin(_) => unreachable!(),
            CardType::Take2 => 50,
            CardType::Buff(BuffType::Freeze) => 10,
        }
    }

    pub fn select(&self, state: &mut GameState, slot: usize) {
        match self {
            CardType::Empty => {}
            CardType::Tower => state.gui.set_cursor_card_effect(slot, self),
            CardType::SellTower => state.gui.set_cursor_card_effect(slot, self),
            CardType::DamageEnemy => state.gui.set_cursor_card_effect(slot, self),
            CardType::Shop => state.overlay_state = Some(Box::new(ShopOverlay::new(slot))),
            CardType::Coin(a) => {
                state.player_mut().gold += (10 as usize).pow(*a as u32);
                state.player_mut().deck.card_used(slot);
            }
            CardType::Take2 => {
                state.player_mut().deck.draw(2);
                state.player_mut().deck.card_used(slot);
            }
            CardType::Buff(BuffType::Freeze) => state.gui.set_cursor_card_effect(slot, self),
        }
    }

    pub fn is_applicable(&self, state: &GameState, x: usize, y: usize) -> bool {
        if state.player().gold < self.activation_cost(state) {
            return false;
        }
        match self {
            CardType::Empty => return false,
            CardType::Tower => {
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
            CardType::Shop => return false,
            CardType::Coin(_) => return false,
            CardType::Take2 => return false,
            CardType::Buff(_) => return state.towers.has_building(x, y),
        }
    }

    pub fn activate(&self, state: &mut GameState, x: usize, y: usize) {
        state.player_mut().gold -= self.activation_cost(state);
        match self {
            CardType::Empty => {}
            CardType::Tower => {
                state.towers.spawn(Tower::new((x, y)));
                state.gui.set_cursor(CursorMode::Hand(0));
                let pos = GameMap::tile_center(x, y);
                state.effects.smoke(pos.x, pos.y)
            }
            CardType::SellTower => {
                state.towers.remove_tower(x, y);
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::DamageEnemy => {
                for e in state.enemies.in_range(GameMap::tile_center(x, y), 80.0) {
                    state.enemies.damage(e, 150, &HashSet::new());
                }
                state.gui.set_cursor(CursorMode::Hand(0));
            }
            CardType::Shop => {}
            CardType::Coin(_) => {}
            CardType::Take2 => {}
            CardType::Buff(b) => {
                state.towers.get_tower_mut(x, y).unwrap().add_buff(*b);
                state.gui.set_cursor(CursorMode::Hand(0));
                let pos = GameMap::tile_center(x, y);
                state.effects.ice(pos.x, pos.y)
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
            Tower,
            Coin(1),
            Coin(1),
            Coin(1),
            Shop,
            Buff(BuffType::Freeze),
        ];
        let discard = vec![];
        Self {
            hand,
            deck,
            discard,
        }
    }

    pub fn discard_all(&mut self) {
        self.discard
            .extend(self.hand.drain(..).filter(|c| c != &CardType::Empty));
    }

    pub fn card_used(&mut self, slot: usize) {
        assert!(self.hand[slot] != CardType::Empty);
        self.discard.push(self.hand[slot]);
        self.hand.remove(slot);
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
