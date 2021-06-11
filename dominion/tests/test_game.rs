// //! Tests for game-related methods and structs

// #[cfg(test)]
// pub mod test_game {
//     use dominion::game::prelude::*;
//     #[test]
//     pub fn test_printsupply() {
//         let mut game = Game::default();
//         game.print_supply();
//     }

//     #[test]
//     pub fn test_vic_cond() {
//         let mut game = Game::default();
//         game.supply.insert(Box::new(Province), 0);
//         assert!(game.victory_met());
//         game.supply.insert(Box::new(Province), 8);
//         game.supply.insert(Box::new(Copper), 0);
//         game.supply.insert(Box::new(Smithy), 0);
//         game.supply.insert(Box::new(Moat), 0);
//         assert!(game.victory_met());
//     }
// }
