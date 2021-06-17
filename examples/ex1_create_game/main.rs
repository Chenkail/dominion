use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callbacks = sample_client::callbacks();
    let player = &game.players[0];

    player.print_status();
    player.print_cards();
    game.action_effects(0, Box::new(Market), callbacks);

    let player = &mut game.players[0];
    player.print_status();
    player.print_cards();
    player.update_coins_in_hand();
    println!("{}", player.resources.coins_in_hand);
}
