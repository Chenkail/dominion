use crate::game::prelude::*;

use serde::{Serialize, Deserialize};

/// A struct representing the game state that a specific player can see.
///
/// These should be generated using `Game::partial_game()` and not constructed
/// by hand.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialGame {
    pub started: bool,
    pub player: Player,
    pub hand_sizes: Vec<usize>,
    pub supply: Supply,
    pub trash: CardDeck,
    pub extras: Supply,
}
