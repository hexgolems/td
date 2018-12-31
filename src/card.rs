use crate::assets::ImgID;
use crate::game_state::GameState;
use crate::gui::CursorMode;
use crate::towers::TowerType;
use rand::prelude::*;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum CardType {
    Empty,
    BuildCannon,
    BuildArchers,
    SellTower,
    DamageEnemy,
}
use self::CardType::*;

impl CardType {
    pub fn get_image_id(&self) -> ImgID {
        match self {
            CardType::Empty => ImgID::EmptySlot,
            CardType::BuildCannon => ImgID::Cannon,
            CardType::BuildArchers => ImgID::Archers,
            CardType::SellTower => ImgID::SellTower,
            CardType::DamageEnemy => ImgID::DamageEnemy,
        }
    }

    pub fn activate(&self, state: &mut GameState) {
        match self {
            CardType::Empty => {}
            CardType::BuildCannon => {
                state.gui.set_cursor(CursorMode::Build {
                    x: 0,
                    y: 0,
                    t: TowerType::Cannon,
                });
            }
            CardType::BuildArchers => {
                state.gui.set_cursor(CursorMode::Build {
                    x: 0,
                    y: 0,
                    t: TowerType::Archers,
                });
            }
            CardType::SellTower => {}
            CardType::DamageEnemy => {}
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
            BuildCannon,
            BuildArchers,
            SellTower,
            DamageEnemy,
            DamageEnemy,
            DamageEnemy,
        ];
        let discard = vec![];
        Self {
            hand,
            deck,
            discard,
        }
    }

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(self.deck.as_mut_slice());
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
