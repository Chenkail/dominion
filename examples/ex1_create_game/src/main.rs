use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let (p1, mut others) = game.players.split_at_mut(1);
    let player = p1.get_mut(0).unwrap();
    let callbacks = sample_client::callbacks();

    player.print_status();
    player.print_cards();
    player.action_effects(&Market, &mut game.supply, &mut game.trash, &mut others, &callbacks);
    player.print_status();
    player.print_cards();
    player.update_coins_in_hand();
    println!("{}", player.resources.coins_in_hand);
}
