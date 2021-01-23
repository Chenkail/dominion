//! Module for single import when defining cards
//!
//! ```
//! use dominion::cards::prelude::*;
//! ```

pub use std::collections::HashMap;
pub use serde::{Serialize, Deserialize};

pub use dominion_macros::*;
pub use crate::game::{card::Card, gamedata::Game, player::Player};
pub use crate::types::*;
pub use CardType::*;
