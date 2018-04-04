use super::{get_turns, Game};
use ncurses::*;
use simple_cards::deck::Deck;

pub fn turn_handler(game: &mut Game, turns: &mut usize, key: i32) {
    {
        let this_player: &mut Deck;
        let other_player: &mut Deck;
        let my_key: i32;
        let next_p1_turn: bool;
        let curr_name: &str;
        let other_name: &str;

        if game.p1_turn == true {
            this_player = &mut game.player1;
            other_player = &mut game.player2;
            my_key = 107;
            next_p1_turn = false;
            curr_name = "Player 1";
            other_name = "player 2";
        } else {
            this_player = &mut game.player2;
            other_player = &mut game.player1;
            my_key = 97;
            next_p1_turn = true;
            curr_name = "Player 2";
            other_name = "player 1";
        }

        if game.face_off {
            if *turns == 0 {
                clear();

                mvprintw(
                    game.max_y / 2,
                    game.max_x / 2 - 21,
                    format!(
                        "{} lost the face off! It's now {}'s turn.",
                        curr_name, other_name
                    ).as_ref(),
                );

                other_player.add_deck(&mut game.pile);

                game.p1_turn = next_p1_turn;
                game.face_off = false;

                let mut cont = getch();

                while cont != 32 {
                    cont = getch();
                }
            } else {
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
}
