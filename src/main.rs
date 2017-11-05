extern crate cards;
extern crate ncurses;
mod player;
use cards::deck::Deck;
use ncurses::*;
use std::process::exit;

fn main() {
    initscr();
    noecho();
    raw();
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    mvprintw(3, (max_x / 2 - 11), "Welcome to Egyptian War!\n");

    mvprintw(max_y / 2 - 1, 1, "Press spacebar to play!");
    
    let mut ch: i32 = 0;
    while ch != 27 {
        if ch == 32 {
            play();
        } else {
            ch = getch();
        }
    }
    endwin();
    exit(0);
}

fn play() {
    clear();
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut player1 = Deck::new_empty();
    let mut player2 = Deck::new_empty();

    let mut deck = Deck::new();
    deck.shuffle();

    mvprintw(1, 1, format!("Player 1: {} cards left", player1.len()).as_ref());
    mvprintw(1, max_x - 23, format!("Player 2: {} cards left", player2.len()).as_ref());
    getch();
    endwin();
    exit(0);
}
