extern crate cards;
use cards::cards::Card;

pub struct Player {
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        let mut cards = Vec::new();
        Player {
            cards
        }
    }
}
