use crate::card_deck::CardDeck;

pub struct Player {
    pub id: usize,
    pub hp: usize,
    pub gold: usize,
    pub deck: CardDeck,
}

impl Player {
    pub fn new(id: usize) -> Self {
        let hp = 10;
        let gold = 300;
        let mut deck = CardDeck::new();
        deck.shuffle();
        deck.draw(5);
        Self { id, deck, hp, gold }
    }

    pub fn debug(id: usize) -> Self {
        let hp = 1337;
        let gold = 9001;
        let mut deck = CardDeck::all();
        deck.shuffle();
        deck.draw(5);
        Self { id, deck, hp, gold }
    }
}
