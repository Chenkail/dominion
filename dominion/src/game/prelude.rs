//! Module for common imports relating to gameplay
//!
//! ```
//! use dominion::game::prelude::*;
//! ```

pub use std::collections::{HashMap, VecDeque};

pub use dominion_macros::*;
pub use uuid::Uuid;

pub use crate::cards::all::*;
pub use crate::game::*;
pub use crate::game::cso::*;
pub use crate::utils;
pub use crate::types::*;
pub use crate::types::CardType::*;
pub use crate::types::AttackTargetType::*;
pub use crate::types::ReactionTrigger::*;
