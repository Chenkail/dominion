use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callbacks = sample_client::callbacks();
    let (p1v, mut others) = game.players.split_at_mut(1);
    let player1 = p1v.get_mut(0).unwrap();

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &mut others, &callbacks);
    player1.gain_to_hand(Box::new(Smithy), &mut game.supply, &mut others, &callbacks);
    player1.print_hand();
    player1.action_phase(&mut game.supply, &mut others, &callbacks);
    player1.print_status();
    player1.print_hand();
    player1.buy_phase(&mut game.supply, &mut others, &callbacks);
    player1.cleanup();

    println!("{:?}", game);
}
