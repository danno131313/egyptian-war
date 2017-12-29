use simple_cards::deck::Deck;
use ncurses::*;
use super::{Game, get_turns};

pub fn turn_handler(mut game: Game, turns: &mut usize, key: usize, curr_player: u32) -> Game {
    {
        let this_player: &mut Deck;
        let other_player: &mut Deck;
        let my_key: usize;
        let next_p1_turn: bool;
        let name: &str;
        if curr_player == 1 {
            this_player = &mut game.player1;
            other_player = &mut game.player2;
            my_key = 117;
            next_p1_turn = false;
            name = "player 2";
        } else {
            this_player = &mut game.player2;
            other_player = &mut game.player1;
            my_key = 97;
            next_p1_turn = true;
            name = "player 1";
        }
        if game.face_off {
            if *turns == 0 {
                clear();
                mvprintw(game.max_y / 2, game.max_x / 2 - 21, format!("You lost the face off! It's now {}'s turn.", name).as_ref());
                other_player.add_deck(&mut game.pile);
                game.p1_turn = next_p1_turn;
                game.face_off = false;
                let mut cont = getch();
                while cont != 32 {
                    cont = getch();
                }
            } else {
                //update_scr(&game);
                mvprintw(game.max_y / 2 - 2, game.max_x / 2 - 14, format!("Face off! {} tries remaining.", turns).as_ref());
                if key == my_key {
                    let curr_card = this_player.draw().expect("Deck is empty!");
                    game.pile.add(curr_card);
                    if curr_card.is_face() {
                        game.p1_turn = next_p1_turn;
                        *turns = get_turns(&curr_card);
                    } else {
                        *turns -= 1;
                    }
                }
            }
        } else if key == my_key {
            let curr_card = this_player.draw().expect("Deck is empty!");
            game.pile.add(curr_card);
            game.p1_turn = next_p1_turn;
            if curr_card.is_face() {
                game.face_off = true;
                *turns = get_turns(&curr_card);
            }
        }
    }
    game
}
