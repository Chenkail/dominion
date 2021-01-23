use dominion::game::prelude::*;

fn main() {
    let mut game = Game::default();
    let mut player = Player::new_with_default_deck(0);

    player.print_state();
    player.print_cards();
    player.action_effects(&Market, &mut game.supply, &mut game.players);
    player.print_state();
    player.print_cards();
    player.update_coins_in_hand();
    println!("{}", player.resources.coins_in_hand);
}
