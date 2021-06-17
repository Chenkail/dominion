//! Module for single import when defining cards
//!
//! ```
//! use dominion::cards::prelude::*;
//! ```

pub use std::collections::HashMap;
pub use serde::{Serialize, Deserialize};

pub use dominion_macros::*;

pub use crate::game::{Card, Game, Player};
pub use crate::callbacks::Callbacks;

pub use crate::types::*;
pub use crate::types::CardType::*;
pub use crate::types::AttackTargetType::*;
pub use crate::types::ReactionTrigger::*;

pub use crate::error::DominionError::{self, *};
