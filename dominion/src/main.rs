mod game;
use game::{cards, gamedata::Game, player::*, traits::*};
use cards::dominion::*;

fn main() {
    let game = Game::default(4);
    let mut player = Player::new();
    player.print_cards();
    player.play_action(&Market);
    player.print_cards();
}
