//! Defines traits for the various card types

use std::hash::{Hash, Hasher};
use crate::game::{gamedata::Game, player::Player};

/// The basic Card trait
///
/// dyn Card implements [Hash] and [Eq] so that Box\<dyn Card\> can be used as keys for a HashMap
pub trait Card {
    /// How much the card costs to buy
    fn cost(&self) -> i32;
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card text (this will often be blank, as is the case with all the cards in the base set)
    fn description(&self) -> &str {
        return "";
    }
}

impl Hash for dyn Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq_ignore_ascii_case(other.name())
    }
}

impl Eq for dyn Card {}

/// Trait for treasure cards
pub trait Treasure: Card {
    /// How many coins the card is worth
    fn value(&self, player: &Player) -> i32;
}

/// Trait for victory cards
pub trait Victory: Card {
    /// How many victory points the card is worth
    fn points(&self, player: &Player) -> i32;
}

/// Trait for curse cards
pub (crate) trait CurseTrait: Card {
    /// How many victory points the card is worth (this should be negative)
    fn points(&self, player: &Player) -> i32;
}

/// Trait for action cards
pub trait Action: Card {
    /// Effects that the Action card has on the person playing it
    fn effects(&self, player: &mut Player, game: &mut Game);
}

/// Trait for attack cards
pub trait Attack: Action {
    /// Effects that the Attack card has
    fn attack(&self, player: &mut Player, game: &mut Game);
}

/// Trait for reaction cards
pub trait Reaction: Card {
    /// Effects that the Reaction card has
    ///
    /// TODO: player boolean flag for Moat immunity?
    fn react(&self, player: &mut Player, game: &mut Game);
}
