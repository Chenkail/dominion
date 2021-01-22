use dominion::game::prelude::*;

fn main() {
    let mut game = Game::default();
    let (mut player1, p1others) = game.player_and_others(0);

    player1.gain_to_hand(Box::new(Market), &mut game.supply, &p1others);
    player1.print_hand();
    for card in &player1.hand {
        card.print_types();
    }

    player1.play_action_from_hand(5, &mut game.supply, &p1others).unwrap();
    player1.print_hand();
    player1.cleanup();

    println!("{:?}", game);
}
