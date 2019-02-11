use crate::card_deck::CardDeck;

pub struct Player {
    pub id: usize,
    pub hp: usize,
    pub gold: usize,
    pub deck: CardDeck,
}

impl Player {
    pub fn new(id: usize, debug: bool) -> Self {
        let mut hp = 10;
        let mut gold = 300;
        let mut deck = CardDeck::new();
        if debug {
            deck = CardDeck::all();
            gold = 9001;
            hp = 1337
        }
        deck.shuffle();
        deck.draw(5);
        Self { id, deck, hp, gold }
    }
}
