use crate::card_deck::CardDeck;
use crate::playing_state::PlayingState;

pub struct Player {
    pub id: usize,
    pub hp: usize,
    pub gold: usize,
    pub mana: f32,
    pub deck: CardDeck,
}

impl Player {
    pub fn new(id: usize) -> Self {
        let hp = 10;
        let gold = 300;
        let mana = 3.0;
        let mut deck = CardDeck::new();
        deck.shuffle();
        deck.draw(5);
        Self {
            id,
            deck,
            hp,
            gold,
            mana,
        }
    }

    pub fn debug(id: usize) -> Self {
        let hp = 1337;
        let gold = 9001;
        let mana = 100.0;
        let mut deck = CardDeck::all();
        deck.shuffle();
        deck.draw(5);
        Self {
            id,
            deck,
            hp,
            gold,
            mana,
        }
    }

    pub fn tick(state: &mut PlayingState) {
        state.player_mut().mana += 0.005;
    }
}
