use crate::card::CardDeck;

pub struct Player {
    pub id: usize,
    pub hp: usize,
    pub gold: usize,
    pub deck: CardDeck,
}

impl Player {
    pub fn new(id: usize) -> Self {
        let mut deck = CardDeck::new();
        deck.shuffle();
        deck.draw(5);
        let hp = 10;
        let gold = 3000;
        Self { id, deck, hp, gold }
    }
}
