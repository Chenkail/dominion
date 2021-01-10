mod game;
use game::{cards, gamedata::Game, player, traits::*};

use cards::*;
use player::Player;

fn main() {
    let game = Game::default(4);
    
}
