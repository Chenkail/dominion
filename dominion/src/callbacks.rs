//! The callbacks that need to be provided when building a client

use crate::game::Card;
use super::prelude::Supply;

use dyn_clonable::clonable;

#[clonable]
pub trait Callbacks: Clone + Send + Sync {
    fn choose_card_from_supply(&self, player_number: usize) -> Option<Box<dyn Card>>;
    fn choose_card_from_hand(&self, message: &str) -> usize;
    fn choose_card_from_hand_opt(&self, message: &str) -> Option<usize>;
    fn choose_cards_from_hand(&self, number_to_choose: usize, message: &str) -> Vec<usize>;
    fn reveal_hand(&self, player_number: usize);
    fn get_player_consent(&self, player_number: usize, prompt: &str) -> bool;
}


// /// Is the player done with this phase
// pub prompt_player_done: FnToBool,
// /// Get an index of a card in hand to choose to play
// pub prompt_card_from_hand: FnToUsize,
// /// Get a list of indices of cards from hand
// pub prompt_indices_from_hand: FnToVecUsize,
// /// Get a list of indices of cards from hand -- up to a certain size
// pub prompt_indices_from_hand_u: FnUsizeToVecUsize,

// /// reveal top x cards of player's discard pile
// /// if reveal whole discard, just pass in discard.len()
// pub reveal_top_discard_pile: FnPlayerUsize,

// /// reveal player's hand
// pub reveal_hand: FnPlayer,

// /// reveal top x cards of the player's draw pile
// /// if reveal whole draw pile, just pass in draw.len()
// pub reveal_top_draw_pile: FnPlayerUsize,

// // callback to get player consent (yes / no)
// pub get_player_consent: FnPlayerToBool,

// pub choose_card_from_supply: FnSupplyToCard,

// // future callbacks to be implemented:

// // Callback for "choose one" effects that takes a vec of strings with descriptions
// // as an argument and returns a usize for the chosen option
// // callback to prompt card from discard,
// // callback to prompt list of indexes from discard
// // callback to prompt list of cards from trash
// // and more as i think of them
// //
// // i'm wondering if we want to collapse all FnPlayerUsize functions
// // into one function that just gets indexes and nothing else considering
// // there's a lot of repeated code here

