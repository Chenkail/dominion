//! Tests for game-related methods and structs

#[cfg(test)]
pub mod test_game {
    use dominion::game::gamedata::*;
    #[test]
    pub fn test_printsupply() {
        let mut game = Game::default();
        game.print_supply();
    }
    
}
