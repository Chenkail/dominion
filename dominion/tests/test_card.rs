//! Tests for specific card implementations.


#[cfg(test)]
pub mod test_card {
    use dominion::game::prelude::*;

    #[test]
    pub fn test_gardens() {
        let mut game = Game::default();
        let callbacks = dominion::sample_client::callbacks();

        supply_add!(game.supply, Gardens, 200);

        let _ = game.gain_to_hand(0, Box::new(Gardens), callbacks);

        let player1 = &game.players[0];
        assert_eq!(player1.victory_points(), 4);
    }
}
