//! Tests for game-related methods and structs

#[cfg(test)]
pub mod test_game {
    use dominion::game::prelude::*;
    #[test]
    pub fn test_print_supply() {
        let mut game = Game::default();
        game.print_supply();
    }

    #[test]
    pub fn test_vic_cond() {
        let mut game = Game::default();

        supply_add!(game.supply, Province, 0);
        assert!(game.victory_met());

        supply_add!(game.supply, Province, 8);
        supply_add!(game.supply, Copper, 0);
        supply_add!(game.supply, Smithy, 0);
        supply_add!(game.supply, Moat, 0);
        assert!(game.victory_met());
    }
}
