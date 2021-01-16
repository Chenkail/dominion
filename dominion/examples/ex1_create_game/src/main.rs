use dominion::game::prelude::*;
use dominion_macros::*;

fn main() {
    let mut game = Game::default();
    let mut player = Player::default();

    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game.supply, &mut game.players);
    player.print_state();
    player.print_cards();
    player.update_coins_in_hand();
    println!("{}", player.resources.coins_in_hand);

    // let e = player.play_action_from_hand(0, &mut game.supply, &mut game.players);
    // println!("Error: {:?}", e);
}
