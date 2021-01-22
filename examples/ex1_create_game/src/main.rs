use dominion::game::prelude::*;

fn main() {
    let mut game = Game::default();
    let (mut player, others) = game.player_and_others(0);

    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game.supply, &others);
    player.print_state();
    player.print_cards();
    player.update_coins_in_hand();
    println!("{}", player.resources.coins_in_hand);
}
