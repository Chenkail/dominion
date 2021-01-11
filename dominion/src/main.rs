mod game;
use game::{cards, gamedata::Game, player::*, traits::*};
use cards::dominion::*;

fn main() {
    let kingdom_cards: Vec<Box<dyn Card>> = vec![Box::new(Cellar), Box::new(Market), Box::new(Merchant), Box::new(Militia), 
                                                    Box::new(Mine), Box::new(Moat), Box::new(Remodel), Box::new(Smithy), 
                                                    Box::new(Village), Box::new(Workshop)];
    let game = Game::new(4, kingdom_cards);
    let mut player = Player::new();
    player.print_cards();
    player.action_effects(&Market);
    player.print_cards();
}
