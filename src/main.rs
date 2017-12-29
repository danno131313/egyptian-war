extern crate simple_cards;
extern crate ncurses;
use simple_cards::deck::Deck;
use simple_cards::cards::Card;
use simple_cards::cards::Value;
use ncurses::*;
use std::process::exit;

struct Game {
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

fn slap_handler(game: &mut Game, winner: u32) {
    let message: String;
    let player: &mut Deck;
    let p1_next_turn: bool;
    let name: &'static str;
    if winner == 1 {
        player = &mut game.player1;
        p1_next_turn = true;
        name = "Player 1";
    } else {
        player = &mut game.player2;
        p1_next_turn = false;
        name = "Player 2";
    }
    if game.pile.len() > 1 {
        if game.pile.show(game.pile.len() - 1).value == game.pile.show(game.pile.len() - 2).value {
            message = "Player 1 slapped a double!".to_string();
            player.add_deck(&mut game.pile);
            game.p1_turn = p1_next_turn;
            game.face_off = false;
        } else if game.pile.len() > 2 && game.pile.show(game.pile.len() -1).value == game.pile.show(game.pile.len() - 3).value {
            message = format!("{} {}", name, "slapped a sandwich!");
            player.add_deck(&mut game.pile);
            game.p1_turn = p1_next_turn;
            game.face_off = false;
        } else {
            message = "Player 1 slapped wrong :(".to_string();
            game.pile.add_back(player.draw().expect("Deck is empty!"));
        }
    } else {
        message = "Player 1 slapped wrong :(".to_string();
        game.pile.add_back(player.draw().expect("Deck is empty!"));
    }
    clear();
    mvprintw(game.max_y / 2, game.max_x / 2 - 13, &message);
    let mut cont = getch();
    while cont != 32 {
        cont = getch();
    }
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
<<<<<<< HEAD
            slap_handler(&mut game, 1);
=======
            let message: &str;
            if pile.len() > 1 {
                if pile.show(pile.len() - 1).value == pile.show(pile.len() - 2).value {
                    message = "Player 1 slapped a double!";
                    player1.add_deck(&mut pile);
                    p1_turn = true;
                    face_off = false;
                } else if pile.len() > 2 && pile.show(pile.len() -1).value == pile.show(pile.len() - 3).value {
                    message = "Player 1 slapped a sandwich!";
                    player1.add_deck(&mut pile);
                    p1_turn = true;
                    face_off = false;
                } else {
                    message = "Player 1 slapped wrong :(";
                    pile.add_bottom(player1.draw().expect("Deck is empty!"));
                }
            } else {
                message = "Player 1 slapped wrong :(";
                pile.add_bottom(player1.draw().expect("Deck is empty!"));
            }
            clear();
            mvprintw(max_y / 2, max_x / 2 - 13, message);
            let mut cont = getch();
            while cont != 32 {
                cont = getch();
            }
>>>>>>> c89f053569782e4e808c57e53c3b4cf32dd243ec
        }


        // 'S' key: player2 slap
        if key == 115 {
<<<<<<< HEAD
            slap_handler(&mut game, 2);
=======
            let message: &str;
            if pile.len() > 1 {
                if pile.show(pile.len() - 1).value == pile.show(pile.len() - 2).value {
                    message = "Player 2 slapped a double!";
                    player2.add_deck(&mut pile);
                    p1_turn = false;
                    face_off = false;
                } else if pile.len() > 2 && pile.show(pile.len() -1).value == pile.show(pile.len() - 3).value {
                    message = "Player 2 slapped a sandwich!";
                    player2.add_deck(&mut pile);
                    p1_turn = false;
                    face_off = false;
                } else {
                    message = "Player 2 slapped wrong :(";
                    pile.add_bottom(player2.draw().expect("Deck is empty!"));
                }
            } else {
                message = "Player 2 slapped wrong :(";
                pile.add_bottom(player2.draw().expect("Deck is empty!"));
            }
            clear();
            mvprintw(max_y / 2, max_x / 2 - 13, message);
            let mut cont = getch();
            while cont != 32 {
                cont = getch();
            }
>>>>>>> c89f053569782e4e808c57e53c3b4cf32dd243ec
        }

        if game.p1_turn {
            if game.face_off {
                if turns == 0 {
                    clear();
                    mvprintw(game.max_y / 2, game.max_x / 2 - 21, "You lost the face off! It's now player 2's turn.");
                    game.player2.add_deck(&mut game.pile);
                    game.p1_turn = false;
                    game.face_off = false;
                    let mut cont = getch();
                    while cont != 32 {
                        cont = getch();
                    }
                } else {
                    update_scr(&game);
                    mvprintw(game.max_y / 2 - 2, game.max_x / 2 - 14, format!("Face off! {} tries remaining.", turns).as_ref());
                    if key == 107 {
                        let curr_card = game.player1.draw().expect("Deck is empty!");
                        game.pile.add(curr_card);
                        if curr_card.is_face() {
                            game.p1_turn = false;
                            turns = get_turns(&curr_card);
                        } else {
                            turns -= 1;
                        }
                    }
                }
            } else if key == 107 {
                let curr_card = game.player1.draw().expect("Deck is empty!");
                game.pile.add(curr_card);
                game.p1_turn = false;
                if curr_card.is_face() {
                    game.face_off = true;
                    turns = get_turns(&curr_card);
                }
            }
        } else {
            if game.face_off {
                if turns == 0 {
                    clear();
                    mvprintw(game.max_y / 2, game.max_x / 2 - 21, "You lost the face off! It's now player 1's turn.");
                    game.player1.add_deck(&mut game.pile);
                    game.p1_turn = true;
                    game.face_off = false;
                    let mut cont = getch();
                    while cont != 32 {
                        cont = getch();
                    }
                } else {
                    update_scr(&game);
                    mvprintw(game.max_y / 2 - 2, game.max_x / 2 - 14, format!("Face off! {} tries remaining.", turns).as_ref());
                    println!("{}", turns);
                    if key == 97 {
                        let curr_card = game.player2.draw().expect("Deck is empty!");
                        game.pile.add(curr_card);
                        if curr_card.is_face() {
                            game.p1_turn = true;
                            turns = get_turns(&curr_card);
                        } else {
                            turns -= 1;
                        }
                    }
                }
            } else if key == 97 {
                let curr_card = game.player2.draw().expect("Deck is empty!");
                game.pile.add(curr_card);
                game.p1_turn = true;
                if curr_card.is_face() {
                    game.face_off = true;
                    turns = get_turns(&curr_card);
                }
            }
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
