use crate::game::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialGame {
    pub started: bool,
    pub player: Player,
    pub hand_sizes: Vec<usize>,
    pub supply: Supply,
    pub trash: CardDeck,
    pub extras: Supply,
}
