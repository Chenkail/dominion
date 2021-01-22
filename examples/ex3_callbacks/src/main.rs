use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callback = sample_client::text_client();
    let (p1v, others) = game.players.split_at_mut(1);
    let player1 = p1v.get_mut(0).unwrap();

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &others);
    player1.action_phase(&mut game.supply, &others, &callback);
    player1.print_state();
    player1.print_hand();
    player1.buy_phase(&mut game.supply, &others, &callback);
    player1.cleanup();

    println!("{:?}", game);
}
