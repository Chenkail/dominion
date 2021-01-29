use dominion::game::prelude::*;
use dominion::sample_client;

fn main() {
    let mut game = Game::default();
    let callbacks = sample_client::callbacks();

    let (p1, mut others) = game.players.split_at_mut(1);
    let player1 = p1.get_mut(0).unwrap();

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &mut game.trash, &mut others, &callbacks);
    player1.print_hand();
    for card in &player1.hand {
        card.print_types();
    }

    player1.resources.actions = 1;
    player1.play_action_from_hand(5, &mut game.supply, &mut game.trash, &mut others, &callbacks).unwrap();
    player1.print_hand();
    player1.cleanup();

    println!("{:?}", game);
}
