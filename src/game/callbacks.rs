//! The callbacks that need to be provided when building a client

use crate::game::player::Player;

pub type FnToBool = Box<dyn Fn() -> bool>;
pub type FnPlayerToi32 = Box<dyn Fn(&Player) -> i32>;
pub type FnToVecUsize = Box<dyn Fn() -> Vec<usize>>;

pub struct Callbacks {
    /// Is the player done with this phase
    pub prompt_player_done: FnToBool,
    /// Get an index of a card in hand to choose to play
    pub prompt_card_from_hand: FnPlayerToi32,
    /// Get a list of indices of cards from hand
    pub prompt_indices_from_hand: FnToVecUsize,
}
