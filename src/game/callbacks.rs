//! The callbacks that need to be provided when building a client

use crate::game::player::Player;

pub type FnToBool = Box<dyn Fn() -> bool>;
pub type FnToUsize = Box<dyn Fn() -> usize>;
pub type FnToVecUsize = Box<dyn Fn() -> Vec<usize>>;
pub type FnPlayerToUsize = Box<dyn Fn(&Player) -> usize>;
pub type FnPlayerToi32 = Box<dyn Fn(&Player) -> i32>;
pub type FnUsizeToVecUsize = Box<dyn Fn(usize) -> Vec<usize>>;
pub type FnPlayerUsize = Box<dyn Fn(&Player, usize)>;

pub struct Callbacks {
    /// Is the player done with this phase
    pub prompt_player_done: FnToBool,
    /// Get an index of a card in hand to choose to play
    pub prompt_card_from_hand: FnToUsize,
    /// Get a list of indices of cards from hand
    pub prompt_indices_from_hand: FnToVecUsize,
    /// Get a list of indices of cards from hand -- up to a certan size
    pub prompt_indices_from_hand_u: FnUsizeToVecUsize,

    
    /// reveal top x cards of player's discard pile
    /// if reveal whole discard, just pass in discard.len()
    pub reveal_top_discard_pile: FnPlayerUsize,

    /// reveal player's hand
    pub reveal_hand: Box<dyn Fn(&Player)>,

    /// reveal top x cards of the player's draw pile
    /// if reveal whole draw pile, just pass in draw.len()
    pub reveal_top_draw_pile: FnPlayerUsize,

    
}
