use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callbacks = sample_client::callbacks();

    game.gain_to_hand(0, Box::new(Market), &callbacks).unwrap();
    game.gain_to_hand(0, Box::new(Smithy), &callbacks).unwrap();

    let player1 = &game.players[0];
    player1.print_hand();
    game.action_phase(0, &callbacks);

    let player1 = &game.players[0];
    player1.print_status();
    player1.print_hand();

    game.buy_phase(0, &callbacks);

    let player1 = &mut game.players[0];
    player1.cleanup();

    println!("{:?}", game);
}
