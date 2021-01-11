use dominion::game::prelude::*;
use dominion_macros::*;

fn main() {
    let kingdom_cards = card_vec![Cellar, Market, Merchant, Militia, Mine, Moat, Remodel, Smithy, Village, Workshop];

    let mut game = Game::new(4, kingdom_cards);
    let mut player = Player::new();
    
    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game);
    player.print_state();
    player.print_cards();
}
