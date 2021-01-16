use dominion::game::prelude::*;
use dominion_macros::*;

fn main() {
    let mut game = Game::default();
    let (p1v, others) = game.players.split_at_mut(1);
    let player1 = p1v.get_mut(0).unwrap();

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &others);
    player1.print_hand();
    player1.play_action_from_hand(5, &mut game.supply, &others);
    player1.print_hand();
}
