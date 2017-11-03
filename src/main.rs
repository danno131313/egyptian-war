extern crate cards;
mod player;
use cards::Deck;
use player::Player;

fn main() {
    println!("Welcome to Egyptian War!");
    let mut deck = Deck::new().shuffle();
    let mut player1 = Player::new();
    println!(deck.draw());
}
