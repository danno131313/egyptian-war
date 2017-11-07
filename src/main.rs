extern crate cards;
extern crate ncurses;
use cards::deck::Deck;
use cards::cards::Card;
use ncurses::*;
use std::process::exit;


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

            play(player1, player2, pile);
        } else {
            ch = getch();
        }
    }
    endwin();
    exit(0);
}

fn play(mut player1: Deck, mut player2: Deck, mut pile: Deck) {
    clear();
    let mut max_y = 0;
    let mut max_x = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    mvprintw(1, 1, format!("Player 1: {} cards left", player1.len()).as_ref());
    mvprintw(1, max_x - 23, format!("Player 2: {} cards left", player2.len()).as_ref());

    if pile.len() == 0 {
        mvprintw(4, 1, "Player 1:");
        mvprintw(5, 3, "A for drawing a card");
        mvprintw(6, 3, "S for slappping");
        mvprintw(8, 1, "Player 1:");
        mvprintw(9, 3, "K for drawing a card");
        mvprintw(10, 3, "L for slappping");
        mvprintw(12, 1, "Player 1 goes first");
        mvprintw(13, 1, "Press spacebar to continue...");

        let mut ch = getch();
        while ch != 32 {
            ch = getch();
        }
        clear();
    }

    let mut game_over = false;
    let mut p1_turn   = true;
    let mut face_off  = false;
    let mut turns     = 0;

    while !game_over {
        update_scr(&player1, &player2, &pile, max_y, max_x);

        let key = getch();

        // 'Esc' key: exit game
        if key == 27 {
            endwin();
            exit(0);
        }

        // 'L' key: player1 slap
        if key == 108 {
            let mut message = "";
            if (pile.len() > 1) {
                if pile.show(pile.len() - 1).value == pile.show(pile.len() - 2).value {
                    message = "Player 1 slapped a double!";
                    player1.add_deck(&mut pile);
                    p1_turn = true;
                } else if pile.len() > 2 && pile.show(pile.len() -1).value == pile.show(pile.len() - 3).value {
                    message = "Player 1 slapped a sandwich!";
                    player1.add_deck(&mut pile);
                    p1_turn = true;
                } else {
                    message = "Player 1 slapped wrong :(";
                    pile.add_back(player1.draw().expect("Deck is empty!"));
                }
            } else {
                message = "Player 1 slapped wrong :(";
                pile.add_back(player1.draw().expect("Deck is empty!"));
            }
            clear();
            mvprintw(max_y / 2, max_x / 2 - 13, message);
            let mut cont = getch();
            while cont != 32 {
                cont = getch();
            }
        }

        // 'S' key: player2 slap
        if key == 115 {
            let mut message = "";
            if (pile.len() > 1) {
                if pile.show(pile.len() - 1).value == pile.show(pile.len() - 2).value {
                    message = "Player 2 slapped a double!";
                    player2.add_deck(&mut pile);
                    p1_turn = false;
                } else if pile.len() > 2 && pile.show(pile.len() -1).value == pile.show(pile.len() - 3).value {
                    message = "Player 2 slapped a sandwich!";
                    player2.add_deck(&mut pile);
                    p1_turn = false;
                } else {
                    message = "Player 2 slapped wrong :(";
                    pile.add_back(player2.draw().expect("Deck is empty!"));
                }
            } else {
                message = "Player 2 slapped wrong :(";
                pile.add_back(player2.draw().expect("Deck is empty!"));
            }
            clear();
            mvprintw(max_y / 2, max_x / 2 - 13, message);
            let mut cont = getch();
            while cont != 32 {
                cont = getch();
            }
        }

        if p1_turn {
            while face_off {
                update_scr(&player1, &player2, &pile, max_y, max_x);
                mvprintw(max_y / 2 - 2, max_x / 2 - 4, format!("Face off! {} tries remaining.", turns).as_ref());
                if key == 107 {
                    let curr_card = player1.draw().expect("Deck is empty!");
                    pile.add(curr_card);
                    if curr_card.is_face() {
                        p1_turn = false;
                        turns = get_turns(&curr_card);
                    }
                }
            }
            // 'K' key: player1 draw
            if key == 107 {
                let curr_card = player1.draw().expect("Deck is empty!");
                pile.add(curr_card);
                p1_turn = false;
            }
        } else {
            // 'A' key: player2 draw
            if key == 97 {
                let curr_card = player2.draw().expect("Deck is empty!");
                pile.add(curr_card);
                p1_turn = true;

            }
        }

        if player1.len() < 14 || player2.len() < 14 {
            game_over = true;
        }
    }
    clear();
    let mut winner = "";
    if player1.len() < 14 {
        winner = "Player 2";
    } else {
        winner = "Player 1";
    }
    mvprintw(max_y / 2, max_x / 2 - 5, "Game Over!");
    mvprintw(max_y / 2 + 2, max_x / 2 - 6, format!("{} wins!", winner).as_ref());

    getch();
    endwin();
    exit(0);
}

fn get_turns(card: &Card) -> usize {
    match card.value {
        Ace => 4,
        King => 3,
        Queen => 2,
        Jack => 1,
        _ => 0,
    }
}

fn update_scr(player1: &Deck, player2: &Deck, pile: &Deck, max_y: i32, max_x: i32, ) {
    clear();
    mvprintw(1, 1, format!("Player 1: {} cards left", player1.len()).as_ref());
    mvprintw(1, max_x - 23, format!("Player 2: {} cards left", player2.len()).as_ref());
    if pile.len() != 0 {
        mvprintw(max_y / 2, max_x / 2 - 6, format!("{}", pile.show(pile.len() - 1)).as_ref());
    }
}
