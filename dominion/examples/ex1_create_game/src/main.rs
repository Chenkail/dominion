use dominion::game::prelude::*;
use dominion_macros::*;

fn main() {
    let kingdom_cards = card_vec![Cellar, Market, Merchant, Militia, Mine, Moat, Remodel, Smithy, Village, Workshop];

    let mut game = Game::new(4, kingdom_cards);
    let mut player = Player::default();

    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game.supply, &mut game.players);
    player.print_state();
    player.print_cards();
    println!("{}", player.count_coins_in_hand());

    let e = player.play_action_from_hand(0, &mut game.supply, &mut game.players);
    println!("Error: {:?}", e);
}
