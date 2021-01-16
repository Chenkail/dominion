use dominion::game::prelude::*;
use dominion_macros::*;

fn main() {
    let mut game = Game::default();
    let mut player1 = game.players.get_mut(0).unwrap();
    // let mut player2 = game.players.get_mut(1).unwrap();
    let mut others = vec![];
    // let mut others = &game.players[1..2];

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &mut others);
    player1.print_hand();
    player1.play_action_from_hand(5, &mut game.supply, &mut others);
    player1.print_hand();
}
