use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callbacks = sample_client::callbacks();

    game.gain_to_hand(0, Box::new(Market), &callbacks);

    let player1 = &mut game.players[0];
    player1.print_hand();
    for card in &player1.hand {
        card.print_types();
    }

    player1.resources.actions = 1;
    game.play_action_from_hand(0, 5, &callbacks).unwrap();

    let player1 = &mut game.players[0];
    player1.print_hand();
    player1.cleanup();

    println!("{:?}", game);
}
