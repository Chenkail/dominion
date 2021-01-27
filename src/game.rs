//! The actual game and its pieces.
//!
//! Dominion includes several moving parts. These include the cards, the
//! players, the cards, and the game itself.

pub mod prelude;
pub mod utils;
pub mod callbacks;

mod cso;
mod card;
mod gamedata;
mod player;

pub use cso::*;
pub use card::Card;
pub use gamedata::Game;
pub use player::Player;
pub use callbacks::Callbacks;
