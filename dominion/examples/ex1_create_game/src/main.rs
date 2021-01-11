use dominion::game::prelude::*;

fn main() {
    let kingdom_cards: Vec<Box<dyn Card>> = 
        vec![Box::new(Cellar), Box::new(Market), Box::new(Merchant), Box::new(Militia), Box::new(Mine), 
            Box::new(Moat), Box::new(Remodel), Box::new(Smithy), Box::new(Village), Box::new(Workshop)];

    let mut game = Game::new(4, kingdom_cards);
    let mut player = Player::new();
    
    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game);
    player.print_state();
    player.print_cards();
}
