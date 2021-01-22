use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callback = sample_client::client();
    let (mut player1, p1others) = game.player_and_others(0);

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &p1others);
    player1.action_phase(&mut game.supply, &p1others, &callback);
    player1.print_state();
    player1.print_hand();
    player1.buy_phase(&mut game.supply, &p1others, &callback);
    player1.cleanup();

    println!("{:?}", game);
}
