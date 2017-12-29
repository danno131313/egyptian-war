use simple_cards::deck::Deck;
use ncurses::*;
use super::Game;

pub fn slap_handler(mut game: Game, winner: u32) -> Game {
    {
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
                message = format!("{} {}", name, "slapped a double!");
                player.add_deck(&mut game.pile);
                game.p1_turn = p1_next_turn;
                game.face_off = false;
            } else if game.pile.len() > 2 && game.pile.show(game.pile.len() -1).value == game.pile.show(game.pile.len() - 3).value {
                message = format!("{} {}", name, "slapped a sandwich!");
                player.add_deck(&mut game.pile);
                game.p1_turn = p1_next_turn;
                game.face_off = false;
            } else {
                message = format!("{} {}", name, "slapped wrong :(");
                game.pile.add_bottom(player.draw().expect("Deck is empty!"));
            }
        } else {
            message = format!("{} {}", name, "slapped wrong :(");
            game.pile.add_bottom(player.draw().expect("Deck is empty!"));
        }
        clear();
        mvprintw(game.max_y / 2, game.max_x / 2 - 13, &message);
        let mut cont = getch();
        while cont != 32 {
            cont = getch();
        }
    }
    game
}
