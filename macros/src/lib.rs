//! Helper macro crate for Dominion.
//!
//! If defining cards, these macros (and several other necessary imports for
//! doing so) can all be included by adding the following line at the top of
//! your file:
//! ```
//! use dominion::cards::prelude::*;
//! ```
//! If making a client for the game, use the following import:
//! ```
//! use dominion::game::prelude::*;
//! ```
//! Otherwise, the macros can be imported directly with
//! ```
//! use dominion_macros::*;
//! ```

mod card_macros;
mod utils;
