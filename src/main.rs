extern crate simple_cards;
extern crate ncurses;

mod turn;
mod slap;

use turn::*;
use slap::*;
use simple_cards::deck::Deck;
use simple_cards::cards::Card;
use simple_cards::cards::Value;
use ncurses::*;
use std::process::exit;

pub struct Game {
    player1: Deck,
    player2: Deck,
    pile: Deck,
    max_y: i32,
    max_x: i32,
    face_off: bool,
    p1_turn: bool,
}

fn main() {
    initscr();
    noecho();
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    raw();

    mvprintw(3, (max_x / 2 - 11), "Welcome to Egyptian War!\n");

    mvprintw(max_y / 2 - 1, 1, "Press spacebar to play!");
    
    let mut ch: i32 = 0;
    while ch != 27 {
        if ch == 32 {
            let mut player1 = Deck::new_empty();
            let mut player2 = Deck::new_empty();
            let mut pile    = Deck::new();

            pile.shuffle();
            while pile.len() != 0 {
                player1.add(pile.draw().expect("Deck is empty!"));
                player2.add(pile.draw().expect("Deck is empty!"));
            }

            let game = Game {
                player1: player1,
                player2: player2,
                pile: pile,
                max_y: max_y,
                max_x: max_x,
                face_off: false,
                p1_turn: true,
            };
            play(game);
        } else {
            ch = getch();
        }
    }
    endwin();
    exit(0);
}

fn play(mut game: Game) {
    clear();
    getmaxyx(stdscr(), &mut game.max_y, &mut game.max_x);

    update_scr(&game);

    if game.pile.len() == 0 {
        mvprintw(4, 1, "Player 1:");
        mvprintw(5, 3, "K for drawing a card");
        mvprintw(6, 3, "L for slapping");
        mvprintw(8, 1, "Player 2:");
        mvprintw(9, 3, "A for drawing a card");
        mvprintw(10, 3, "S for slapping");
        mvprintw(12, 1, "Player 1 goes first");
        mvprintw(13, 1, "Press spacebar to continue...");

        let mut ch = getch();
        while ch != 32 {
            ch = getch();
        }
        clear();
    }

    let mut game_over = false;
    let mut turns     = 0;

    while !game_over {
        update_scr(&game);

        if game.face_off {
            mvprintw(game.max_y / 2 - 2, game.max_x / 2 - 14, format!("Face off! {} tries remaining.", turns).as_ref());
        }

        let key = getch();

        // 'Esc' key: exit game
        if key == 27 {
            endwin();
            exit(0);
        }

        // 'L' key: player1 slap
        if key == 108 {
            game = slap_handler(game, 1);
        }

        // 'S' key: player2 slap
        if key == 115 {
            game = slap_handler(game, 2);
        }

        if game.p1_turn {
            game = turn_handler(game, &mut turns, 117, 1);
        } else {
            game = turn_handler(game, &mut turns, 97, 2);
        }

        if game.player1.len() < 14 || game.player2.len() < 14 {
            game_over = true;
        }
    }
    clear();
    let winner: &str;
    if game.player1.len() < 10 {
        winner = "Player 2";
    } else {
        winner = "Player 1";
    }
    mvprintw(game.max_y / 2, game.max_x / 2 - 5, "Game Over!");
    mvprintw(game.max_y / 2 + 2, game.max_x / 2 - 6, format!("{} wins!", winner).as_ref());

    getch();
    endwin();
    exit(0);
}

fn get_turns(card: &Card) -> usize {
    match card.value {
        Value::Ace => 4,
        Value::King => 3,
        Value::Queen => 2,
        Value::Jack => 1,
        _ => 0,
    }
}

fn update_scr(game: &Game) {
    clear();
    mvprintw(1, 1, format!("Player 2: {} cards left", game.player2.len()).as_ref());
    mvprintw(1, game.max_x - 23, format!("Player 1: {} cards left", game.player1.len()).as_ref());
    if game.pile.len() != 0 {
        mvprintw(game.max_y / 2, game.max_x / 2 - 6, format!("{}", game.pile.show(game.pile.len() - 1)).as_ref());
    }
}
